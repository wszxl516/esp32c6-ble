use esp_idf_hal::sys::temperature_sensor_obj_t;
use esp_idf_svc::sys::{
    esp, temperature_sensor_abs_threshold_config_t, temperature_sensor_config_t,
    temperature_sensor_enable, temperature_sensor_get_celsius, temperature_sensor_install,
    temperature_sensor_set_absolute_threshold,
};
use std::ptr::{addr_of, addr_of_mut};

pub struct Tmep {
    sensor: *mut temperature_sensor_obj_t,
}
impl Tmep {
    pub fn new() -> anyhow::Result<Self> {
        let mut temp_sensor = 0 as *mut temperature_sensor_obj_t;
        let mut temp_sensor_config = temperature_sensor_config_t::default();
        temp_sensor_config.range_min = 10;
        temp_sensor_config.range_max = 50;
        temp_sensor_config.clk_src = 9;
        unsafe {
            esp!(temperature_sensor_install(
                addr_of!(temp_sensor_config),
                addr_of_mut!(temp_sensor)
            ))
        }?;
        let threshold_cfg = temperature_sensor_abs_threshold_config_t {
            high_threshold: 50.0,
            low_threshold: 0.0,
        };
        unsafe {
            esp!(temperature_sensor_set_absolute_threshold(
                temp_sensor,
                addr_of!(threshold_cfg)
            ))
        }?;
        unsafe { temperature_sensor_enable(temp_sensor) };
        Ok(Self {
            sensor: temp_sensor,
        })
    }
    pub fn get(&self) -> anyhow::Result<f32> {
        let mut value = 0f32;
        unsafe { esp!(temperature_sensor_get_celsius(self.sensor, &mut value)) }?;
        Ok(value)
    }
}
