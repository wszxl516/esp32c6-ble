use std::ffi::c_long;
use super::BleDevice;
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::NimbleProperties;
use log::{debug, info};
use esp_idf_hal::sys::{clock_settime, time_t, timespec};

pub fn init_time(device: &mut BleDevice) -> anyhow::Result<()> {
    debug!("init time!");
    let service = device.add_service("TIME", 0xff1, BleUuid::Uuid16(0xfff0))?;

    let time_service = service.lock().create_characteristic(
        BleUuid::Uuid16(0xfff1),
        NimbleProperties::WRITE | NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    time_service.lock().on_write(move |args| {
        let data = args.recv_data();
        debug!("time_service {:?}", data);
        if data.len() >= 8 {
            info!("time sync succeed!");
            let seconds= u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
            let t = timespec {
                tv_sec: seconds as time_t,
                tv_nsec: (seconds / 1000000000) as c_long,
            };
            unsafe {
                clock_settime(1, &t as *const timespec);
            }
        }
    });
    Ok(())
}
