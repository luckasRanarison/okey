use anyhow::Result;
use evdev::{Device, InputEvent, uinput::VirtualDevice};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};

pub trait EventProxy {
    fn emit(&mut self, events: &[InputEvent]) -> Result<()>;
    fn wait(&mut self, timeout: u16) -> Result<()>;
}

#[derive(Debug)]
pub struct InputProxy {
    epoll: Epoll,
    event_buffer: [EpollEvent; 1],
    virtual_device: VirtualDevice,
}

impl InputProxy {
    pub fn try_from_device(device: &Device) -> Result<Self> {
        let name = format!("{} (virtual)", device.name().unwrap_or("Unknown device"));
        let virtual_device = VirtualDevice::builder()?
            .name(&name)
            .with_keys(device.supported_keys().unwrap_or_default())?
            .build()?;

        let epoll = Epoll::new(EpollCreateFlags::EPOLL_CLOEXEC)?;
        let event = EpollEvent::new(EpollFlags::EPOLLIN, 0);
        let event_buffer = [EpollEvent::empty(); 1];

        epoll.add(device, event)?;

        Ok(Self {
            epoll,
            event_buffer,
            virtual_device,
        })
    }
}

impl EventProxy for InputProxy {
    fn emit(&mut self, events: &[InputEvent]) -> Result<()> {
        self.virtual_device.emit(events)?;
        Ok(())
    }

    fn wait(&mut self, timeout: u16) -> Result<()> {
        self.epoll.wait(&mut self.event_buffer, timeout)?;
        Ok(())
    }
}
