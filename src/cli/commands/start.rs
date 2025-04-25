use std::thread;

use anyhow::{Result, anyhow};

use crate::{
    core::EventProxy,
    fs::{config::read_config, device::find_device_by_name},
};

pub fn start(config_path: Option<String>) -> Result<()> {
    let parsed = read_config(config_path)?;

    let handles = parsed.keyboards.into_iter().map(|keyboard| {
        let defaults = parsed.defaults.clone();

        thread::spawn(|| -> Result<()> {
            let device = find_device_by_name(&keyboard.name)?;

            let mut event_proxy = match device {
                Some(device) => EventProxy::new(device, keyboard, defaults),
                None => Err(anyhow!("Device not found")),
            }?;

            event_proxy.init_hook()
        })
    });

    for handle in handles {
        handle.join().unwrap()?
    }

    Ok(())
}
