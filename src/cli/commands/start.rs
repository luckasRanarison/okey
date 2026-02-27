use std::{fs::File, io, os::fd::AsRawFd, process, thread};

use anyhow::{anyhow, Result};
use nix::unistd::{self, ForkResult};

use crate::{
    core::{InputProxy, KeyAdapter},
    fs::{config::read_config, device::find_device_by_name},
};

pub fn start(config_path: Option<String>) -> Result<()> {
    let parsed = read_config(config_path)?;

    let handles = parsed.keyboards.into_iter().map(|keyboard| {
        let defaults = parsed.defaults.clone();

        thread::spawn(move || -> Result<()> {
            let mut device = find_device_by_name(&keyboard.name)?
                .ok_or(anyhow!("Device not found: {}", keyboard.name))?;

            let mut proxy = InputProxy::try_from_device(&device)?;
            let mut adapter = KeyAdapter::new(keyboard, defaults, &mut proxy);

            adapter.hook(&mut device)
        })
    });

    simple_logger::init()?;

    for handle in handles {
        handle.join().unwrap()?
    }

    Ok(())
}

pub fn start_daemon(config_path: Option<String>) -> Result<()> {
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("okey daemon started in the background... (PID: {child})");
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
