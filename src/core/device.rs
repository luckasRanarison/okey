use std::fs;

use anyhow::Result;
use evdev::Device;

pub fn find_device_by_name(name: &str) -> Result<Option<Device>> {
    let files = fs::read_dir("/dev/input")?;

    for file in files {
        let entry = file?;

        if !entry.file_name().to_string_lossy().contains("event") {
            continue;
        }

        let device = Device::open(entry.path())?;

        if device.name().is_some_and(|value| value == name) {
            return Ok(Some(device));
        }
    }

    Ok(None)
}
