mod color;
mod rmt;
pub mod smart_led;

use std::ops::Deref;
use crate::common::data::{Color, Gradient, State};
use crate::led::smart_led::Ws2812Esp32Rmt;
use esp_idf_hal::gpio::IOPin;
use esp_idf_hal::rmt::CHANNEL0;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{RGB, RGB8};
use smart_leds_trait::SmartLedsWrite;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use std::time::Duration;
use crate::fs::config::{CONFIG};

pub struct LedState<'d> {
    driver: Ws2812Esp32Rmt<'d>,
    receiver: Receiver<State>,
    state: State,
}

impl LedState<'_> {
    pub fn new(
        channel: CHANNEL0,
        pin: impl IOPin,
        receiver: Receiver<State>,
    ) -> anyhow::Result<Self> {
        let state = match CONFIG.deref() {
            None => State::Color(Color::Rgb(RGB::new(0, 0, 0))),
            Some(config) => State::Gradient(config.led)
        };
        Ok(Self {
            driver: Ws2812Esp32Rmt::new(channel, pin)?,
            receiver,
            state,
        })
    }

    pub fn color(&mut self, color: Color, count: usize) {
        let color = std::iter::repeat(match color {
            Color::Rgb(rgb) => rgb,
            Color::Hsv(hsv) => hsv2rgb(hsv),
        })
        .take(count);
        self.driver.write(color).unwrap()
    }
    pub fn gradient_hsv(&mut self, start: u8, end: u8, sat: u8, val: u8, interval: u64) {
        for hue in start..=end {
            self.color(Color::Hsv(Hsv { hue, sat, val }), 1);
            sleep(Duration::from_millis(interval));
        }
    }
    pub fn gradient_rgb(&mut self, start: u8, end: u8, rgb_type: u8, interval: u64) {
        for i in start..=end {
            let color = match rgb_type {
                0 => RGB8::new(i, 0, 0),
                1 => RGB8::new(0, i, 0),
                2 => RGB8::new(0, 0, i),
                _ => unreachable!(),
            };
            self.color(Color::Rgb(color), 1);
            sleep(Duration::from_millis(interval));
        }

        for i in (start..=end).rev() {
            let color = match rgb_type {
                0 => RGB8::new(i, 0, 0),
                1 => RGB8::new(0, i, 0),
                2 => RGB8::new(0, 0, i),
                _ => unreachable!(),
            };
            self.color(Color::Rgb(color), 1);
            sleep(Duration::from_millis(interval));
        }
    }
    pub fn run(&mut self, state: State) {
        match state {
            State::Color(color) => self.color(color, 1),
            State::Gradient(gradient) => match gradient {
                Gradient::Rgb(start, end, t, interval) => {
                    self.gradient_rgb(start, end, t, interval as u64);
                }
                Gradient::Hsv(start, end, sat, val, interval) => {
                    self.gradient_hsv(start, end, sat, val, interval as u64);
                }
            },
            _ => {}
        }
    }
    pub fn start(&mut self) -> anyhow::Result<()> {
        match self.receiver.try_recv() {
            Ok(state) => {self.state = state},
            Err(_) => {}
        };
        self.run(self.state);
        Ok(())
    }
}
