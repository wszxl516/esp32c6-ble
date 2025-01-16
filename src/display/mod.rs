mod backend;
pub mod st7735s;

use crate::common::temp::Tmep;
use crate::display::backend::EspBackend;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::OriginDimensions;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::RgbColor;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{SpiDevice, MODE_0};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio0, IOPin, PinDriver};
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_hal::spi::{Dma, SpiConfig, SpiDeviceDriver, SPI2};
use slint::{Color, SharedString};
use st7735s::{Orientation, ST7735};
use std::default::Default;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::common::{get_date, MemInfo};
slint::include_modules!();

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
        .baudrate(80.MHz().into())
        .write_only(true)
        .data_mode(MODE_0)
        .queue_size(2)
        .polling(true);
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

pub fn show_ui<D>(display: D, receiver: Receiver<String>) -> anyhow::Result<(), anyhow::Error>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions + 'static,
    D::Error: std::fmt::Debug,
{
    slint::platform::set_platform(Box::new(EspBackend::new(display)))
        .expect("backend already initialized");
    let root = MainWindow::new()
        .map_err(|e| anyhow::Error::msg(format!("init slint ui failed: {}", e)))?;
    let strong = root.clone_strong();
    let t = Tmep::new()?;
    let mut old_msg = String::with_capacity(256);
    let mut current_msg = String::with_capacity(256);
    let timer = slint::Timer::default();
    let mut mem = MemInfo::new();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(200),
        move || {
            match receiver.try_recv() {
                Ok(msg) => current_msg = msg,
                Err(_) => {}
            };

            let date = get_date();
            let temp = t.get().unwrap_or(0.0);
            mem.fetch();
            let (free, total) = mem.kb();
            let time_str = format!(
                "{}\nTemp: {:.0}C Mem: {:.01}%",
                date.format("%d %b %a %H:%M:%S"),
                temp,  ((total - free) / total) * 100.0
            );
            strong.invoke_set_info_title(SharedString::from(time_str), 32);

            if current_msg != old_msg {
                strong.invoke_set_info_text(
                    SharedString::from(current_msg.clone()),
                    Color::from_rgb_u8(255, 255, 255),
                    22,
                )
            }
            old_msg.clear();
            old_msg.push_str(current_msg.as_str());
        },
    );

    root.run()
        .map_err(|e| anyhow::Error::msg(format!("run slint ui failed: {}", e)))?;
    Ok(())
}
