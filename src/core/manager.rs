// use std::{collections::HashMap, time::Instant};

use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::schema::KeyboardConfig;

use super::map::KeyCodeMap;

// #[derive(Debug)]
// pub struct KeyState {
//     timeout: u16,
//     timestamp: Instant,
//     released: bool,
// }

#[derive(Debug)]
pub struct KeyManager {
    mappings: KeyCodeMap,
    // pressed_keys: HashMap<u16, KeyState>,
    virtual_device: VirtualDevice,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig, virtual_device: VirtualDevice) -> Self {
        let mappings = KeyCodeMap::new(config.keys.unwrap_or_default());

        Self {
            mappings,
            virtual_device,
            // pressed_keys: HashMap::with_capacity(5),
        }
    }

    pub fn process_event(&mut self, event: InputEvent) -> Result<()> {
        if event.event_type() != EventType::KEY {
            return Ok(());
        }

        let code = event.code();
        let code = self.mappings.get_mapped_keycode(&code).unwrap_or(code);

        self.virtual_device
            .emit(&[InputEvent::new(event.event_type().0, code, event.value())])?;

        // match event.value() {
        //     1 => {}
        //     2 => {}
        //     0 => {}
        //     _ => {}
        // }

        Ok(())
    }

    pub fn next(&mut self) -> Result<()> {
        // if self.pressed_keys.is_empty() {
        //     return Ok(());
        // }

        Ok(())
    }
}
