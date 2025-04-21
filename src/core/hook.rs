use anyhow::Result;
use evdev::{Device, uinput::VirtualDevice};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};

use crate::config::schema::KeyboardConfig;

use super::manager::KeyManager;

#[derive(Debug)]
pub struct EventHandler {
    device: Device,
    key_manager: KeyManager,
}

impl EventHandler {
    pub fn new(device: Device, config: KeyboardConfig) -> Result<Self> {
        let virtual_device = VirtualDevice::builder()?
            .name("okey virtual keyboard")
            .with_keys(device.supported_keys().unwrap_or_default())?
            .build()?;

        let key_manager = KeyManager::new(config, virtual_device);

        Ok(Self {
            device,
            key_manager,
        })
    }

    pub fn init_hook(&mut self) -> Result<()> {
        self.device.grab()?;
        self.device.set_nonblocking(true)?;

        let epoll = Epoll::new(EpollCreateFlags::EPOLL_CLOEXEC)?;
        let event = EpollEvent::new(EpollFlags::EPOLLIN, 0);
        let epoll_timeout = 10_u16; // milliseconds

        let mut epoll_buffer = [EpollEvent::empty(); 1];

        epoll.add(&self.device, event)?;

        loop {
            epoll.wait(&mut epoll_buffer, epoll_timeout)?;

            if let Ok(events) = self.device.fetch_events() {
                for event in events {
                    self.key_manager.process_event(event)?;
                }
            }

            self.key_manager.next()?;
        }
    }
}
