use std::{fs, thread};

use anyhow::{Result, anyhow};

use crate::{config::schema::Config, core::EventProxy, fs::device::find_device_by_name};

pub fn start(config: &str) -> Result<()> {
    let config = fs::read_to_string(config)?;
    let parsed: Config = serde_yaml::from_str(&config)?;

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
