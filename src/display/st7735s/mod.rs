#![allow(dead_code)]
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::raw::{RawData, RawU16},
    prelude::*,
    primitives::Rectangle,
};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi;

use instruction::Instruction;

pub mod instruction;

/// ST7735 driver to connect to TFT displays.
#[derive(Copy, Clone)]
pub struct ST7735<SPI, DC, RST, BG>
where
    SPI: spi::SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    BG: OutputPin,
{
    /// SPI
    spi: SPI,

    /// Data/command pin.
    dc: DC,

    /// Reset pin.
    rst: Option<RST>,

    ///backlight
    bg: Option<BG>,

    /// Whether the display is RGB (true) or BGR (false)
    rgb: bool,

    /// Whether the colours are inverted (true) or not (false)
    inverted: bool,

    /// Global image offset
    dx: u16,
    dy: u16,
    width: usize,
    height: usize,
    orientation: Orientation,
}

/// Display orientation.
#[derive(Clone, Copy, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum Orientation {
    Portrait = 0x00,
    Landscape = 0x60,
    PortraitSwapped = 0xC0,
    LandscapeSwapped = 0xA0,
}

impl From<Orientation> for u8 {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Portrait => 0x00,
            Orientation::Landscape => 0x60,
            Orientation::PortraitSwapped => 0xC0,
            Orientation::LandscapeSwapped => 0xA0,
        }
    }
}

impl<SPI, DC, RST, BG> ST7735<SPI, DC, RST, BG>
where
    SPI: spi::SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    BG: OutputPin,
{
    pub fn new(
        spi: SPI,
        dc: DC,
        rst: Option<RST>,
        bg: Option<BG>,
        rgb: bool,
        inverted: bool,
        width: u32,
        height: u32,
        orientation: Orientation,
    ) -> Self {
        let display = ST7735 {
            spi,
            dc,
            rst,
            bg,
            rgb,
            inverted,
            dx: 0,
            dy: 0,
            width: width as _,
            height: height as _,
            orientation,
        };

        display
    }

    /// Runs commands to initialize the display.
    pub fn init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayNs,
    {
        self.set_bg(true);
        self.hard_reset(delay)?;
        self.write_command(Instruction::SWRESET, &[])?;
        delay.delay_ms(200);
        self.write_command(Instruction::SLPOUT, &[])?;
        delay.delay_ms(200);
        self.write_command(Instruction::FRMCTR1, &[0x01, 0x2C, 0x2D])?;
        self.write_command(Instruction::FRMCTR2, &[0x01, 0x2C, 0x2D])?;
        self.write_command(Instruction::FRMCTR3, &[0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D])?;
        self.write_command(Instruction::INVCTR, &[0x07])?;
        self.write_command(Instruction::PWCTR1, &[0xA2, 0x02, 0x84])?;
        self.write_command(Instruction::PWCTR2, &[0xC5])?;
        self.write_command(Instruction::PWCTR3, &[0x0A, 0x00])?;
        self.write_command(Instruction::PWCTR4, &[0x8A, 0x2A])?;
        self.write_command(Instruction::PWCTR5, &[0x8A, 0xEE])?;
        self.write_command(Instruction::VMCTR1, &[0x0E])?;
        self.write_command(Instruction::GAMSET, &[0x02])?;
        if self.inverted {
            self.write_command(Instruction::INVON, &[])?;
        } else {
            self.write_command(Instruction::INVOFF, &[])?;
        }
        if self.rgb {
            self.write_command(Instruction::MADCTL, &[0x00])?;
        } else {
            self.write_command(Instruction::MADCTL, &[0x08])?;
        }
        self.write_command(Instruction::COLMOD, &[0x05])?;
        self.write_command(Instruction::GMCTRP1, &[0x02, 0x1c, 0x07, 0x12, 0x37, 0x32, 0x29,
            0x2d, 0x29, 0x25, 0x2b, 0x39, 0x00, 0x01, 0x03, 0x10])?;
        self.write_command(Instruction::GMCTRN1, &[0x03, 0x1d, 0x07, 0x06, 0x2e, 0x2c, 0x29,
            0x2d, 0x2e, 0x2e, 0x37, 0x3f, 0x00, 0x00, 0x02, 0x10])?;
        self.write_command(Instruction::NORON, &[])?;
        self.write_command(Instruction::DISPON, &[])?;
        delay.delay_ms(100);
        self.set_orientation(self.orientation, self.width as u32, self.height as u32)?;
        delay.delay_ms(200);
        Ok(())
    }
    #[inline(always)]
    pub fn set_bg(&mut self, state: bool) {
        match &mut self.bg {
            None => {}
            Some(ref mut bg) => {
                if state {
                    bg.set_high().unwrap()
                } else {
                    bg.set_high().unwrap()
                }
            }
        }
    }
    #[inline(always)]
    pub fn hard_reset<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayNs,
    {
        if let Some(rst) = &mut self.rst {
            rst.set_high().map_err(|_| ())?;
            delay.delay_ms(10);
            rst.set_low().map_err(|_| ())?;
            delay.delay_ms(10);
            rst.set_high().map_err(|_| ())?;
        }
        Ok(())
    }
    #[inline(always)]
    fn write_command(&mut self, command: Instruction, params: &[u8]) -> Result<(), ()> {
        self.dc.set_low().map_err(|_| ())?;
        self.spi.write(&[command as u8]).map_err(|_| ())?;
        if !params.is_empty() {
            self.start_data()?;
            self.write_data(params)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn start_data(&mut self) -> Result<(), ()> {
        self.dc.set_high().map_err(|_| ())
    }
    #[inline(always)]
    fn write_data(&mut self, data: &[u8]) -> Result<(), ()> {
        self.spi.write(data).map_err(|_| ())
    }

    #[inline(always)]
    fn write_word(&mut self, value: u16) -> Result<(), ()> {
        self.write_data(&value.to_be_bytes())
    }

    fn write_words_buffered(&mut self, words: impl IntoIterator<Item = u16>) -> Result<(), ()> {
        let mut buffer = [0; 32];
        let mut index = 0;
        for word in words {
            let as_bytes = word.to_be_bytes();
            buffer[index] = as_bytes[0];
            buffer[index + 1] = as_bytes[1];
            index += 2;
            if index >= buffer.len() {
                self.write_data(&buffer)?;
                index = 0;
            }
        }
        self.write_data(&buffer[0..index])
    }

    pub fn set_orientation(
        &mut self,
        orientation: Orientation,
        width: u32,
        height: u32,
    ) -> Result<(), ()> {
        if self.rgb {
            self.write_command(Instruction::MADCTL, &[u8::from(orientation)])?;
        } else {
            self.write_command(Instruction::MADCTL, &[u8::from(orientation)| 0x08])?;
        }
        self.width = width as _;
        self.height = height as _;
        Ok(())
    }

    /// Sets the global offset of the displayed image
    #[inline(always)]
    pub fn set_offset(&mut self, dx: u16, dy: u16) {
        self.dx = dx;
        self.dy = dy;
    }

    /// Sets the address window for the display.
    pub fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) -> Result<(), ()> {
        self.write_command(Instruction::CASET, &[])?;
        self.start_data()?;
        self.write_word(sx + self.dx)?;
        self.write_word(ex + self.dx)?;
        self.write_command(Instruction::RASET, &[])?;
        self.start_data()?;
        self.write_word(sy + self.dy)?;
        self.write_word(ey + self.dy)
    }

    /// Sets a pixel color at the given coords.
    pub fn set_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()> {
        self.set_address_window(x, y, x, y)?;
        self.write_command(Instruction::RAMWR, &[])?;
        self.start_data()?;
        self.write_word(color)
    }

    /// Writes pixel colors sequentially into the current drawing window
    pub fn write_pixels<P: IntoIterator<Item = u16>>(&mut self, colors: P) -> Result<(), ()> {
        self.write_command(Instruction::RAMWR, &[])?;
        self.start_data()?;
        for color in colors {
            self.write_word(color)?;
        }
        Ok(())
    }
    pub fn write_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        colors: P,
    ) -> Result<(), ()> {
        self.write_command(Instruction::RAMWR, &[])?;
        self.start_data()?;
        self.write_words_buffered(colors)
    }

    /// Sets pixel colors at the given drawing window
    pub fn set_pixels<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels(colors)
    }

    pub fn set_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels_buffered(colors)
    }
}

impl<SPI, DC, RST, BG> DrawTarget for ST7735<SPI, DC, RST, BG>
where
    SPI: spi::SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    BG: OutputPin,
{
    type Color = Rgb565;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Only draw pixels that would be on screen
            if coord.x >= 0
                && coord.y >= 0
                && coord.x < self.width as i32
                && coord.y < self.height as i32
            {
                self.set_pixel(
                    coord.x as u16,
                    coord.y as u16,
                    RawU16::from(color).into_inner(),
                )?;
            }
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        // Clamp area to drawable part of the display target
        let drawable_area = area.intersection(&Rectangle::new(Point::zero(), self.size()));

        if drawable_area.size != Size::zero() {
            self.set_pixels_buffered(
                drawable_area.top_left.x as u16,
                drawable_area.top_left.y as u16,
                (drawable_area.top_left.x + (drawable_area.size.width - 1) as i32) as u16,
                (drawable_area.top_left.y + (drawable_area.size.height - 1) as i32) as u16,
                area.points()
                    .zip(colors)
                    .filter(|(pos, _color)| drawable_area.contains(*pos))
                    .map(|(_pos, color)| RawU16::from(color).into_inner()),
            )?;
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.set_pixels_buffered(
            0,
            0,
            self.width as u16 - 1,
            self.height as u16 - 1,
            core::iter::repeat(RawU16::from(color).into_inner())
                .take((self.width * self.height) as usize),
        )
    }
}

impl<SPI, DC, RST, BG> OriginDimensions for ST7735<SPI, DC, RST, BG>
where
    SPI: spi::SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    BG: OutputPin,
{
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}
