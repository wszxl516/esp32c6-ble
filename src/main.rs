pub mod ble;
pub mod common;
pub mod display;
pub mod fs;
pub mod led;

use crate::ble::led::init_led;
use crate::ble::time::init_time;
use crate::ble::uart::init_uart;
use crate::ble::BleDevice;
use crate::common::data::State;
use crate::display::{setup_display, show_ui};
use crate::led::LedState;
use esp_idf_hal::peripherals::Peripherals;
use log::LevelFilter;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::log::set_target_level("*", LevelFilter::Trace)?;
    let peripherals = Peripherals::take()?;
    let (state_sender, state_receiver) = std::sync::mpsc::channel::<State>();
    let state_sender = state_sender.clone();
    thread::Builder::new()
        .stack_size(4096)
        .name(String::from("LED"))
        .spawn(move || {
            let mut led_state = LedState::new(
                peripherals.rmt.channel0,
                peripherals.pins.gpio8,
                state_receiver,
            )
            .unwrap();
            loop {
                led_state.start().unwrap_or(());
                sleep(Duration::from_millis(10))
            }
        })
        .unwrap();
    let mut device = BleDevice::new("TEST", state_sender.clone())?;
    let (uart_sender, uart_receiver) = std::sync::mpsc::channel::<String>();

    init_uart(&mut device, uart_sender)?;
    init_led(&mut device, state_sender.clone())?;
    init_time(&mut device)?;
    device.start()?;
    thread::Builder::new()
        .stack_size(1024 * 32)
        .name(String::from("DISPLAY"))
        .spawn(move || {
            let display = setup_display(
                peripherals.spi2,
                peripherals.pins.gpio4,
                peripherals.pins.gpio0,
                peripherals.pins.gpio5,
                peripherals.pins.gpio6,
                peripherals.pins.gpio7,
                peripherals.pins.gpio1,
            )
            .expect("setup display failed");
            show_ui(display, uart_receiver).unwrap();
        })
        .unwrap();
    loop {
        sleep(Duration::from_millis(1000));
    }
}
