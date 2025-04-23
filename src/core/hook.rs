use anyhow::Result;
use evdev::{Device, EventType, uinput::VirtualDevice};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};

use crate::config::schema::{DefaultConfig, KeyboardConfig};

use super::manager::KeyManager;

#[derive(Debug)]
pub struct EventEmitter {
    device: Device,
    key_manager: KeyManager,
}

impl EventEmitter {
    pub fn new(device: Device, config: KeyboardConfig, general: DefaultConfig) -> Result<Self> {
        let key_manager = KeyManager::new(config, general);

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
        let epoll_timeout = 1_u16; // milliseconds

        let mut epoll_buffer = [EpollEvent::empty(); 1];

        epoll.add(&self.device, event)?;

        let mut virtual_device = VirtualDevice::builder()?
            .name("okey virtual keyboard")
            .with_keys(self.device.supported_keys().unwrap_or_default())?
            .build()?;

        loop {
            epoll.wait(&mut epoll_buffer, epoll_timeout)?;

            if let Ok(events) = self.device.fetch_events() {
                for event in events {
                    if event.event_type() == EventType::KEY {
                        self.key_manager.process_event(event, &mut virtual_device)?;
                    }
                }
            }

            self.key_manager.post_process(&mut virtual_device)?;
        }
    }
}
