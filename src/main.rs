pub mod ble;
pub mod led;
pub mod display;
pub mod common;

use crate::ble::uart::init_uart;
use crate::ble::BleDevice;
use crate::led::{LedState};
use esp_idf_hal::peripherals::Peripherals;
use log::LevelFilter;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::ble::led::init_led;
use crate::ble::time::init_time;
use crate::common::data::State;
use crate::display::{draw_text, setup_display};

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
        .spawn(move|| {
            let mut led_state = LedState::new(
                peripherals.rmt.channel0,
                peripherals.pins.gpio8,
                state_receiver,
            )
            .unwrap();
            loop {
                led_state.start().unwrap_or(());
                sleep(Duration::from_millis(50))
            }
        })
        .unwrap();
    let mut device = BleDevice::new("TEST", state_sender.clone())?;
    let (uart_sender, uart_receiver) = std::sync::mpsc::channel::<String>();

    init_uart(&mut device, uart_sender)?;
    init_led(&mut device,  state_sender.clone())?;
    init_time(&mut device)?;
    device.start()?;
    thread::Builder::new()
        .stack_size(1024 * 32)
        .name(String::from("DISPLAY"))
        .spawn(move|| {
            let mut display = setup_display(
                peripherals.spi2,
                peripherals.pins.gpio4,
                peripherals.pins.gpio0,
                peripherals.pins.gpio5,
                peripherals.pins.gpio6,
                peripherals.pins.gpio7,
                peripherals.pins.gpio1
            ).expect("setup display failed");
            draw_text(&mut display, uart_receiver);
        })
        .unwrap();
    loop {
        sleep(Duration::from_millis(1000));
    }
}
