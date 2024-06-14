use super::BleDevice;
use crate::common::data::{Gradient, State};
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::NimbleProperties;
use log::debug;
use std::sync::mpsc::Sender;

pub fn init_led(device: &mut BleDevice, sender: Sender<State>) -> anyhow::Result<()> {
    debug!("init uart!");
    let service = device.add_service("LED", 0xff1, BleUuid::Uuid16(0xffff))?;

    let led = service.lock().create_characteristic(
        BleUuid::Uuid16(0xfff1),
        NimbleProperties::WRITE | NimbleProperties::READ | NimbleProperties::NOTIFY,
    );

    led.lock().on_write(move |args| {
        let data = args.recv_data();
        if data.len() == 5 {
            sender
                .send(State::Gradient(Gradient::Hsv(
                    data[0], data[1], data[2], data[3], data[4],
                )))
                .unwrap();
        } else if data.len() == 4 {
            sender
                .send(State::Gradient(Gradient::Rgb(
                    data[0], data[1], data[2], data[3],
                )))
                .unwrap();
        }
        debug!("rx {:?}", args.recv_data());
    });
    Ok(())
}
