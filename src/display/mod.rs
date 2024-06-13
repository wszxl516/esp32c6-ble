pub mod st7735s;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{Dimensions, Point, Size};
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::RgbColor;
use embedded_graphics_core::primitives::Rectangle;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{SpiDevice, MODE_0};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio0, IOPin, PinDriver};
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_hal::spi::{Dma, SpiConfig, SpiDeviceDriver, SPI2};
use st7735s::{Orientation, ST7735};
use std::default::Default;
use std::sync::mpsc::{Receiver};
use std::thread::sleep;
use std::time::{Duration};
use chrono::{FixedOffset, Local};
use embedded_graphics::mono_font::ascii::{FONT_6X9, FONT_8X13};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{Primitive, Transform};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics_core::Drawable;
use embedded_text::alignment::{HorizontalAlignment, VerticalAlignment};
use embedded_text::style::{HeightMode, TextBoxStyleBuilder};
use embedded_text::TextBox;
use crate::common::temp::Tmep;

const DISPLAY_WIDTH: u32 = 160;
const DISPLAY_HEIGHT: u32 = 128;
pub fn setup_display(
    spi2: SPI2,
    clk: impl IOPin,
    cs: impl IOPin,
    sdo: impl IOPin,
    rst: impl IOPin,
    dc: impl IOPin,
    bg: impl IOPin,
) -> anyhow::Result<ST7735<impl SpiDevice, impl OutputPin, impl OutputPin, impl OutputPin>> {
    let sdi = Option::<Gpio0>::None;
    let driver_config = DriverConfig::new()
        .dma(Dma::Auto((DISPLAY_HEIGHT * DISPLAY_WIDTH) as usize))
        .intr_flags(Default::default());
    let spi_config = SpiConfig::new()
        .baudrate(40.MHz().into())
        .write_only(true)
        .data_mode(MODE_0)
        .queue_size(2);
    let spi =
        SpiDeviceDriver::new_single(spi2, clk, sdo, sdi, Some(cs), &driver_config, &spi_config)?;
    let mut display = ST7735::new(
        spi,
        PinDriver::output(dc)?,
        Some(PinDriver::output(rst)?),
        Some(PinDriver::output(bg)?),
        true,
        false,
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        Orientation::Landscape,
    );
    display.hard_reset(&mut FreeRtos).unwrap();
    display.init(&mut FreeRtos).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    display.set_offset(0, 0);
    Ok(display)
}


pub fn draw_text<D>(display: &mut D, receiver: Receiver<String>)
    where D: DrawTarget<Color = Rgb565>, D::Error: std::fmt::Debug
{
    let character_style = MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Left)
        .paragraph_spacing(2)
        .vertical_alignment(VerticalAlignment::Top)
        .trailing_spaces(true)
        .leading_spaces(true)
        .build();

    let text_font = MonoTextStyle::new(&FONT_6X9, Rgb565::new(0,0,255));

    let bounds = Rectangle::new(
        Point::zero() + Point::new(0, (text_font.line_height() * 2) as i32),
        Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut interval = 900u16;
    let t = Tmep::new().unwrap();
    let time_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();
    let temp_style = TextStyleBuilder::new()
        .alignment(Alignment::Right)
        .baseline(Baseline::Top)
        .build();
    let mut old_msg = String::with_capacity(256);
    let mut current_msg = String::with_capacity(256);
    loop {
        let temp = t.get().unwrap();
        match receiver.try_recv() {
            Ok(msg) => {
                current_msg = msg
            }
            Err(_) => {}
        };

        if current_msg != old_msg {
            Rectangle::new(bounds.top_left, bounds.size)
                .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0,0,0)))
                .draw(display).unwrap();
            let text_box = TextBox::with_textbox_style(&current_msg, bounds, character_style, textbox_style);
            text_box.draw(display).unwrap();

        }
        old_msg.clear();
        old_msg.push_str(current_msg.as_str());

        if interval ==  900 {
            let date = Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
            let time_str = format!("{}", date.format("%d %b %a %H:%M:%S"));
            let time_text = Text::with_text_style(
                &time_str,
                Point::zero(),
                text_font,
                time_style
            );

            let time_text_dimensions = time_text.bounding_box();
            Rectangle::new(time_text_dimensions.top_left, Size::new(display.bounding_box().size.width, time_text_dimensions.size.height))
                .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0,0,0)))
                .draw(display).unwrap();
            time_text.draw(display).unwrap();

            let temp_str = format!("{}C", temp);
            let mut temp_text = Text::with_text_style(
                &temp_str,
                Point::zero(),
                text_font,
                temp_style
            );
            temp_text.translate_mut(Point::new(
                DISPLAY_WIDTH  as i32,
                0,
            ));
            temp_text.draw(display).unwrap();
        }
        interval += 100;
        if interval > 900{
            interval = 0
        }
        sleep(Duration::from_millis(100))
    }
}
