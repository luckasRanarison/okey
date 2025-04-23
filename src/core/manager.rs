// use std::{collections::HashMap, time::Instant};

use std::time::Instant;

use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::{
    schema::{KeyAction, KeyboardConfig, LayerConfig},
    utils::{KeyCodeMap, LayerMap, TapDanceMap},
};

use super::{
    event::{IntoInputEvent, IntoInputEvents},
    key::{KeyResult, PressedKey, PressedKeyResult},
};

#[derive(Debug)]
pub struct KeyManager {
    mappings: KeyCodeMap,
    tap_dances: TapDanceMap,
    // layer_manager: LayerManager,
    pressed_keys: Vec<PressedKey>,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig) -> Self {
        let mappings = KeyCodeMap::new(config.keys.unwrap_or_default());
        let tap_dances = TapDanceMap::new(config.tap_dances.unwrap_or_default());
        // let layer_manager = LayerManager::new(config.layers);

        Self {
            mappings,
            tap_dances,
            pressed_keys: Vec::new(),
            // layer_manager,
        }
    }

    pub fn process_event(
        &mut self,
        event: InputEvent,
        virtual_device: &mut VirtualDevice,
    ) -> Result<()> {
        let code = self.mappings.map(&event.code());
        // let code = self.layer_manager.map(&code);
        let value = event.value();

        let result = match value {
            1 => self.handle_press(code),
            2 => self.handle_hold(code),
            _ => self.handle_release(code),
        };

        self.dispatch_result(result, value, virtual_device)
    }

    pub fn post_process(&mut self, virtual_device: &mut VirtualDevice) -> Result<()> {
        if self.pressed_keys.is_empty() {
            return Ok(());
        }

        let now = Instant::now();
        let mut processed = Vec::new();

        for (idx, state) in self.pressed_keys.iter().enumerate() {
            if let Some(result) = state.get_dance_action(now) {
                virtual_device.emit(&result.to_events())?;
                processed.push(idx);
            }
        }

        for &idx in processed.iter().rev() {
            self.pressed_keys.remove(idx);
        }

        Ok(())
    }

    fn dispatch_result(
        &mut self,
        result: KeyResult,
        event_value: i32,
        virtual_device: &mut VirtualDevice,
    ) -> Result<()> {
        match result {
            KeyResult::KeyCode(code) => {
                virtual_device.emit(&[code.to_event(event_value)])?;
            }
            KeyResult::KeyMacro(codes) => {
                virtual_device.emit(&codes.to_events())?;
            }
            KeyResult::None => {}
        }

        Ok(())
    }

    fn handle_press(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                if let Some(config) = self.tap_dances.get(&value) {
                    let key = PressedKey {
                        code: value,
                        timeout: config.timeout,
                        timestamp: Instant::now(),
                        released: false,
                        result: PressedKeyResult::TapDance {
                            tap: config.tap.clone(),
                            hold: config.hold.clone(),
                        },
                    };

                    self.pressed_keys.push(key);

                    KeyResult::None
                } else {
                    KeyResult::KeyCode(code)
                }
            }
            KeyAction::Macro(codes) => KeyResult::KeyMacro(codes),
        }
    }

    fn handle_hold(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(code) => KeyResult::KeyCode(code),
            KeyAction::Macro(_) => KeyResult::None,
        }
    }

    fn handle_release(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                if let Some(state) = self.pressed_keys.iter_mut().find(|s| s.code == value) {
                    state.released = true;
                    KeyResult::None
                } else {
                    KeyResult::KeyCode(code)
                }
            }
            _ => KeyResult::None,
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
