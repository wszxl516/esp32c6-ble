use std::io::Read;
use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::common::data::{Gradient};
use crate::fs::spiffs::SPIFS;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub led: Gradient,
    pub date_fixed_offset: i32,
}
impl Config{
    pub fn from_file() -> Option<Config>{
        SPIFS.check().ok()?;
        let mut f = std::fs::File::open("/data/config.json").ok()?;
        let mut buffer = String::with_capacity(512);
        f.read_to_string(&mut buffer).ok()?;
        match serde_json::from_str::<Config>(&buffer) {
            Ok(config) =>Some(config),
            Err(_) => {
                None
            }
        }
    }
}

lazy_static!{
    pub static ref CONFIG: Option<Config> = {
        let config = Config::from_file();
        debug!("CONFIG: {:?}", config);
        config
    };
}