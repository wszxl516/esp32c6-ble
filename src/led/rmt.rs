#![allow(dead_code)]
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::{PinState, Pulse, RmtChannel, Symbol, TxRmtDriver};
use esp_idf_hal::sys::EspError;
use esp_idf_hal::units::Hertz;
use std::time::Duration;

/// T0H duration time (0 code, high voltage time)
const WS2812_T0H_NS: Duration = Duration::from_nanos(400);
/// T0L duration time (0 code, low voltage time)
const WS2812_T0L_NS: Duration = Duration::from_nanos(850);
/// T1H duration time (1 code, high voltage time)
const WS2812_T1H_NS: Duration = Duration::from_nanos(800);
/// T1L duration time (1 code, low voltage time)
const WS2812_T1L_NS: Duration = Duration::from_nanos(450);

/// Converter to a sequence of RMT items.
#[repr(C)]
struct Ws2812Esp32RmtItemEncoder {
    /// The RMT item that represents a 0 code.
    bit0: (Pulse, Pulse),
    /// The RMT item that represents a 1 code.
    bit1: (Pulse, Pulse),
}

impl Ws2812Esp32RmtItemEncoder {
    fn new(clock_hz: Hertz) -> Result<Self, EspError> {
        let (t0h, t0l, t1h, t1l) = (
            Pulse::new_with_duration(clock_hz, PinState::High, &WS2812_T0H_NS)?,
            Pulse::new_with_duration(clock_hz, PinState::Low, &WS2812_T0L_NS)?,
            Pulse::new_with_duration(clock_hz, PinState::High, &WS2812_T1H_NS)?,
            Pulse::new_with_duration(clock_hz, PinState::Low, &WS2812_T1L_NS)?,
        );

        Ok(Self {
            bit0: (t0h, t0l),
            bit1: (t1h, t1l),
        })
    }

    #[inline]
    fn encode_iter<'a, 'b, T>(&'a self, src: T) -> impl Iterator<Item = Symbol> + Send + 'a
    where
        'b: 'a,
        T: Iterator<Item = u8> + Send + 'b,
    {
        src.flat_map(move |v| {
            (0..(u8::BITS as usize)).map(move |i| {
                if v & (1 << (7 - i)) != 0 {
                    Symbol::new(self.bit1.0, self.bit1.1)
                } else {
                    Symbol::new(self.bit0.0, self.bit0.1)
                }
            })
        })
    }
}

/// WS2812 ESP32 RMT driver wrapper.
pub struct Ws2812Esp32RmtDriver<'d> {
    /// TxRMT driver.
    tx: TxRmtDriver<'d>,
    /// `u8`-to-`rmt_item32_t` Encoder
    encoder: Ws2812Esp32RmtItemEncoder,
}

impl<'d> Ws2812Esp32RmtDriver<'d> {
    pub fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'd,
        pin: impl Peripheral<P = impl OutputPin> + 'd,
    ) -> anyhow::Result<Self> {
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;

        let clock_hz = tx.counter_clock()?;
        let encoder = Ws2812Esp32RmtItemEncoder::new(clock_hz)?;

        Ok(Self { tx, encoder })
    }

    pub fn write_blocking<'a, 'b, T>(&'a mut self, pixel_sequence: T) -> anyhow::Result<()>
    where
        'b: 'a,
        T: Iterator<Item = u8> + Send + 'b,
    {
        let signal = self.encoder.encode_iter(pixel_sequence);
        self.tx.start_iter_blocking(signal)?;
        Ok(())
    }

    pub fn write<'b, T>(&'static mut self, pixel_sequence: T) -> anyhow::Result<()>
    where
        T: Iterator<Item = u8> + Send + 'static,
    {
        let signal = self.encoder.encode_iter(pixel_sequence);
        self.tx.start_iter(signal)?;
        Ok(())
    }
}
