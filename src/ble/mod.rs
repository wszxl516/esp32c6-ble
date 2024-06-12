#![allow(dead_code)]

pub mod led;
pub mod uart;
pub mod time;

use esp32_nimble::enums::{AuthReq, SecurityIOCap};
use esp32_nimble::utilities::mutex::Mutex;
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::{BLEAdvertisementData, BLEDevice, BLEServer, BLEService};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::task::block_on;
use crate::common::data::{BLACK_STATE, GREEN_STATE, State};

pub struct BleDevice {
    device: &'static mut BLEDevice,
    server: &'static mut BLEServer,
    sender: Sender<State>,
}

impl BleDevice {
    pub fn new(name: &str, sender: Sender<State>) -> anyhow::Result<Self> {
        let device = BLEDevice::take();
        BLEDevice::set_device_name(name)?;
        device
            .security()
            .set_auth(AuthReq::Bond)
            .set_passkey(123456)
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();
        let server = device.get_server();
        let s = sender.clone();
        server.on_connect(move |_, desc| {
            log::info!("connect to {:?}", desc);
            for _ in 0..3 {
                s.send(State::Color(GREEN_STATE)).unwrap();
                sleep(Duration::from_millis(100));
                s.send(State::Color(BLACK_STATE)).unwrap()
            }
        });
        server.on_disconnect(move |_, desc| {
            log::info!("disconnect to {:?}", desc);
        });

        Ok(Self {
            device,
            server,
            sender,
        })
    }

    #[inline(always)]
    pub fn add_service(
        &mut self,
        name: &str,
        appearance: u16,
        uuid: BleUuid,
    ) -> anyhow::Result<Arc<Mutex<BLEService>>> {
        let service = self.server.create_service(uuid);
        self.device
            .get_advertising()
            .lock()
            .scan_response(true)
            .set_data(
                BLEAdvertisementData::new()
                    .name(name)
                    .appearance(appearance)
                    .add_service_uuid(service.lock().uuid()),
            )?;
        Ok(service)
    }
    #[inline(always)]
    pub fn get_service(&mut self, uuid: BleUuid) -> Option<&Arc<Mutex<BLEService>>> {
        block_on(async { self.server.get_service(uuid).await.and(None) })
    }
    pub fn connected(&self) -> bool {
        self.server.connected_count() > 0
    }

    pub fn start(&self) -> anyhow::Result<()> {
        self.device.get_advertising().lock().start()?;
        Ok(())
    }
}
