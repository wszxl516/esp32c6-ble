use super::BleDevice;
use crate::common::{DeviceID, get_date, MemInfo};
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::NimbleProperties;
use log::debug;
use std::sync::mpsc::Sender;

const UART_SERVICE_UUID: &str = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E";
const UART_RX_CHAR_UUID: &str = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E";
const UART_TX_CHAR_UUID: &str = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E";
pub fn init_uart(device: &mut BleDevice, sender: Sender<String>) -> anyhow::Result<()> {
    debug!("init uart!");
    let service = device.add_service(
        "UART",
        0xff1,
        BleUuid::from_uuid128_string(UART_SERVICE_UUID).unwrap(),
    )?;

    let rx = service.lock().create_characteristic(
        BleUuid::from_uuid128_string(UART_RX_CHAR_UUID).unwrap(),
        NimbleProperties::WRITE,
    );

    let tx = service.lock().create_characteristic(
        BleUuid::from_uuid128_string(UART_TX_CHAR_UUID).unwrap(),
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    tx.lock()
        .on_read(move |args, _| debug!("tx {:?}", args.value()));
    rx.lock().on_write(move |args| {
        let data = Vec::from(args.recv_data());
        let msg = match std::str::from_utf8(&data) {
            Ok(cmd) => cmd_wrapper(cmd),
            Err(_) => "unknown".to_string(),
        };
        tx.lock().set_value(msg.as_bytes());
        sender.send(msg).unwrap();
        debug!("rx {:?}", args.recv_data());
        tx.lock().notify();
    });
    Ok(())
}

fn cmd_wrapper(cmd: &str) -> String {
    match cmd {
        "date" => {
            let date = get_date();
            format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
        }
        "mem" => {
            let mut mem = MemInfo::new();
            mem.fetch();
            format!("{:.2}kb/{:.2}kb", mem.kb().0, mem.kb().1)
        }
        "mac" => {
            format!("{}", DeviceID::get())
        }
        _ => "unknown".to_string(),
    }
}
