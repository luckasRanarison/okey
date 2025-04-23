use std::{fs, thread};

use anyhow::{Result, anyhow};
use clap::Parser;
use okey::{
    cli::{Cli, Command},
    config::schema::Config,
    core::EventEmitter,
    fs::device::find_device_by_name,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { config } => {
            let config = fs::read_to_string(config)?;
            let parsed: Config = serde_yaml::from_str(&config)?;
            let mut handles = Vec::new();

            for keyboard in parsed.keyboards {
                let defaults = parsed.defaults.clone();

                let handle = thread::spawn(|| -> Result<()> {
                    let device = find_device_by_name(&keyboard.name)?;

                    if let Some(device) = device {
                        Ok(EventEmitter::new(device, keyboard, defaults)?.init_hook()?)
                    } else {
                        Err(anyhow!("Device not found"))
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap()?
            }
        }
    }

    Ok(())
}
