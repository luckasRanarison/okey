use std::{fs::File, io, os::fd::AsRawFd, process, thread};

use anyhow::{Result, anyhow};
use nix::unistd::{self, ForkResult};

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

pub fn start_daemon(config_path: Option<String>) -> Result<()> {
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("okey daemon started in the background... (PID: {})", child);
            process::exit(0);
        }
        Ok(ForkResult::Child) => {
            unistd::setsid()?;
            unistd::chdir("/")?;

            let dev_null = File::open("/dev/null")?;
            let dev_null_fd = dev_null.as_raw_fd();

            unistd::dup2(dev_null_fd, io::stdin().as_raw_fd())?;
            unistd::dup2(dev_null_fd, io::stdout().as_raw_fd())?;
            unistd::dup2(dev_null_fd, io::stderr().as_raw_fd())?;

            start(config_path)?;
        }
        Err(err) => Err(err)?,
    }

    Ok(())
}
