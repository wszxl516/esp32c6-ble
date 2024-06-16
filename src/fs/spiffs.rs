use std::ffi::{CString};
use esp_idf_svc::sys::{esp, esp_spiffs_check, esp_spiffs_format, esp_spiffs_info, esp_spiffs_mounted, esp_vfs_spiffs_conf_t, esp_vfs_spiffs_register, esp_vfs_spiffs_unregister, EspError};
use lazy_static::lazy_static;

pub struct SpiFs{
    partition_label: CString
}


impl SpiFs {
    pub fn register(path: &str, label: &str, max_files: usize) -> anyhow::Result<Self, EspError>{
        let p = CString::new(path).unwrap();
        let l = CString::new(label).unwrap();
        let mut conf = esp_vfs_spiffs_conf_t{
            base_path: p.as_ptr(),
            partition_label: l.as_ptr(),
            max_files,
            format_if_mount_failed: false,
        };
        unsafe {esp!(esp_vfs_spiffs_register(&mut conf))}?;
        Ok(Self{partition_label: CString::new(label).unwrap()})
    }

    pub fn unregister(&self) -> anyhow::Result<(), EspError>{
        unsafe { esp!(esp_vfs_spiffs_unregister(self.partition_label.as_ptr())) }
    }

    pub fn mounted(&self) -> bool {
        unsafe { esp_spiffs_mounted(self.partition_label.as_ptr())}
    }

    pub fn format(&self) -> anyhow::Result<(), EspError> {
        unsafe { esp!(esp_spiffs_format(self.partition_label.as_ptr()))}
    }

    pub fn check(&self) -> anyhow::Result<(), EspError>{
        unsafe { esp!(esp_spiffs_check(self.partition_label.as_ptr()))}
    }

    pub fn info(&self) -> anyhow::Result<(usize, usize), EspError>{
        let mut used = 0usize;
        let mut total = 0usize;
        match unsafe { esp!(esp_spiffs_info(self.partition_label.as_ptr(), &mut total, &mut used))}{
            Ok(_) => Ok((total, used)),
            Err(e) => Err(e)
        }
    }

}
lazy_static!{
    pub static ref SPIFS: SpiFs = {
        SpiFs::register("/data", "data", 65)
        .map_err(|e|anyhow::Error::msg(format!("{}", e)))
        .expect("Failed to init SpiFs from data partition!")
    };
}

