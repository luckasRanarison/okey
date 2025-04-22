// use std::{collections::HashMap, time::Instant};

use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::{
    schema::{KeyAction, KeyboardConfig, LayerConfig},
    utils::{KeyCodeMap, LayerMap},
};

use super::key::{KeyMacro, KeyResult};

#[derive(Debug)]
pub struct KeyManager {
    mappings: KeyCodeMap,
    // layer_manager: LayerManager,
    virtual_device: VirtualDevice,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig, virtual_device: VirtualDevice) -> Self {
        let mappings = KeyCodeMap::new(config.keys.unwrap_or_default());
        // let layer_manager = LayerManager::new(config.layers);

        Self {
            mappings,
            virtual_device,
            // layer_manager,
        }
    }

    pub fn process_event(&mut self, event: InputEvent) -> Result<()> {
        if event.event_type() != EventType::KEY {
            return Ok(());
        }

        let code = self.mappings.map(&event.code());
        // let code = self.layer_manager.map(&code);
        let value = event.value();

        let result = match value {
            1 => self.handle_press(code),
            2 => self.handle_hold(code),
            _ => self.handle_release(code),
        };

        match result {
            KeyResult::KeyCode(code) => {
                self.virtual_device
                    .emit(&[InputEvent::new(EventType::KEY.0, code, value)])?;
            }

            KeyResult::KeyMacro(key_macro) => {
                self.virtual_device.emit(&key_macro.into_events())?;
            }

            KeyResult::Layer => {}
            KeyResult::None => {}
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<()> {
        // if self.pressed_keys.is_empty() {
        //     return Ok(());
        // }

        Ok(())
    }

    fn handle_press(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(keycode) => KeyResult::KeyCode(keycode.value()),
            KeyAction::Macro(codes) => KeyResult::KeyMacro(KeyMacro::from_keycodes(codes)),
        }
    }

    fn handle_hold(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(keycode) => KeyResult::KeyCode(keycode.value()),
            KeyAction::Macro(_) => KeyResult::None,
        }
    }

    fn handle_release(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(keycode) => KeyResult::KeyCode(keycode.value()),
            KeyAction::Macro(_) => KeyResult::None,
        }
    }
}

// #[derive(Debug)]
// struct LayerManager {
//     layer_map: LayerMap,
//     layer_stack: Vec<u16>,
// }
//
// impl LayerManager {
//     fn new(config: Option<LayerConfig>) -> Self {
//         Self {
//             layer_map: config.map(LayerMap::from).unwrap_or_default(),
//             layer_stack: Vec::new(),
//         }
//     }
//
//     fn map(&self, code: &u16) -> u16 {
//         *code
//     }
//
//     fn on_press(&mut self, code: &u16) {}
//
//     fn on_release(&mut self, code: &u16) {}
// }
