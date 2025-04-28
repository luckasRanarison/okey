use std::fs;

use anyhow::Result;
use evdev::Device;

pub fn find_device_by_name(name: &str) -> Result<Option<Device>> {
    let devices = find_input_devices()?;

    let device = devices
        .into_iter()
        .find(|dev| dev.name().is_some_and(|value| value == name));

    Ok(device)
}

pub fn find_input_devices() -> Result<Vec<Device>> {
    let files = fs::read_dir("/dev/input")?;
    let mut results = Vec::new();

    for file in files {
        let entry = file?;

        if !entry.file_name().to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(entry.path()) {
            if device.supported_keys().is_some() {
                results.push(device);
            }
        }
    }

    Ok(results)
}
