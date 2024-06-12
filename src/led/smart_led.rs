#![allow(dead_code)]
use crate::led::color::{LedPixelColor, LedPixelColorGrb24, LedPixelColorImpl};
use crate::led::rmt::Ws2812Esp32RmtDriver;
use esp_idf_hal::{gpio::OutputPin, peripheral::Peripheral, rmt::RmtChannel};
use smart_leds_trait::{SmartLedsWrite, RGB8, RGBW};
use std::marker::PhantomData;

/// 8-bit RGBW (RGB + white)
pub type RGBW8 = RGBW<u8, u8>;

impl<
        const N: usize,
        const R_ORDER: usize,
        const G_ORDER: usize,
        const B_ORDER: usize,
        const W_ORDER: usize,
    > From<RGB8> for LedPixelColorImpl<N, R_ORDER, G_ORDER, B_ORDER, W_ORDER>
{
    fn from(x: RGB8) -> Self {
        Self::new_with_rgb(x.r, x.g, x.b)
    }
}

impl<
        const N: usize,
        const R_ORDER: usize,
        const G_ORDER: usize,
        const B_ORDER: usize,
        const W_ORDER: usize,
    > From<RGBW8> for LedPixelColorImpl<N, R_ORDER, G_ORDER, B_ORDER, W_ORDER>
{
    fn from(x: RGBW8) -> Self {
        Self::new_with_rgbw(x.r, x.g, x.b, x.a.0)
    }
}

pub struct LedPixelEsp32Rmt<'d, CSmart, CDev>
where
    CDev: LedPixelColor + From<CSmart>,
{
    driver: Ws2812Esp32RmtDriver<'d>,
    phantom: PhantomData<(CSmart, CDev)>,
}

impl<'d, CSmart, CDev> LedPixelEsp32Rmt<'d, CSmart, CDev>
where
    CDev: LedPixelColor + From<CSmart>,
{
    pub fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'd,
        pin: impl Peripheral<P = impl OutputPin> + 'd,
    ) -> anyhow::Result<Self> {
        let driver = Ws2812Esp32RmtDriver::<'d>::new(channel, pin)?;
        Ok(Self {
            driver,
            phantom: Default::default(),
        })
    }
}

impl<
        'd,
        CSmart,
        const N: usize,
        const R_ORDER: usize,
        const G_ORDER: usize,
        const B_ORDER: usize,
        const W_ORDER: usize,
    > LedPixelEsp32Rmt<'d, CSmart, LedPixelColorImpl<N, R_ORDER, G_ORDER, B_ORDER, W_ORDER>>
where
    LedPixelColorImpl<N, R_ORDER, G_ORDER, B_ORDER, W_ORDER>: From<CSmart>,
{
    pub fn write_nocopy<T, I>(&mut self, iterator: T) -> anyhow::Result<()>
    where
        T: IntoIterator<Item = I>,
        I: Into<CSmart>,
        <T as IntoIterator>::IntoIter: Send,
    {
        self.driver
            .write_blocking(iterator.into_iter().flat_map(|color| {
                let c =
                    LedPixelColorImpl::<N, R_ORDER, G_ORDER, B_ORDER, W_ORDER>::from(color.into());
                c.0
            }))?;
        Ok(())
    }
}

impl<'d, CSmart, CDev> SmartLedsWrite for LedPixelEsp32Rmt<'d, CSmart, CDev>
where
    CDev: LedPixelColor + From<CSmart>,
{
    type Error = anyhow::Error;
    type Color = CSmart;

    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<Self::Color>,
    {
        let pixel_data = iterator.into_iter().fold(Vec::new(), |mut vec, color| {
            vec.extend_from_slice(CDev::from(color.into()).as_ref());
            vec
        });
        self.driver.write_blocking(pixel_data.into_iter())?;
        Ok(())
    }
}

pub type Ws2812Esp32Rmt<'d> = LedPixelEsp32Rmt<'d, RGB8, LedPixelColorGrb24>;
