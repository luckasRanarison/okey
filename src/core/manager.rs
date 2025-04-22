// use std::{collections::HashMap, time::Instant};

use std::{collections::HashSet, time::Instant};

use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::{
    schema::{KeyAction, KeyboardConfig, LayerConfig},
    utils::{KeyCodeMap, LayerMap, TapDanceMap},
};

use super::key::{KeyMacro, KeyResult, KeyState};

#[derive(Debug)]
pub struct KeyManager {
    mappings: KeyCodeMap,
    tap_dances: TapDanceMap,
    // layer_manager: LayerManager,
    pressed_keys: Vec<KeyState>,
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
                virtual_device.emit(&[InputEvent::new(EventType::KEY.0, code, value)])?;
            }
            KeyResult::KeyMacro(key_macro) => {
                virtual_device.emit(&key_macro.into_events())?;
            }
            KeyResult::KeyPressed(state) => self.pressed_keys.push(state),

            KeyResult::Layer => {}
            KeyResult::None => {}
        }

        Ok(())
    }

    pub fn next(&mut self, virtual_device: &mut VirtualDevice) -> Result<()> {
        if self.pressed_keys.is_empty() {
            return Ok(());
        }

        let now = Instant::now();
        let mut to_remove = Vec::new();

        for (i, state) in self.pressed_keys.iter().enumerate() {
            let elapsed = now.duration_since(state.timestamp).as_millis();

            if elapsed > state.timeout as u128 {
                self.emit_key_action(state.hold.clone(), virtual_device)?;
                to_remove.push(i);
            } else if state.released {
                self.emit_key_action(state.tap.clone(), virtual_device)?;
                to_remove.push(i);
            }
        }

        for &i in to_remove.iter().rev() {
            self.pressed_keys.remove(i);
        }

        Ok(())
    }

    fn emit_key_action(&self, action: KeyAction, virtual_device: &mut VirtualDevice) -> Result<()> {
        match action {
            KeyAction::KeyCode(code) => Ok(virtual_device.emit(&[
                InputEvent::new(EventType::KEY.0, code.value(), 1),
                InputEvent::new(EventType::KEY.0, code.value(), 0),
            ])?),
            KeyAction::Macro(keycodes) => {
                Ok(virtual_device.emit(&KeyMacro::from_keycodes(keycodes).into_events())?)
            }
        }
    }

    fn handle_press(&mut self, action: KeyAction) -> KeyResult {
        match action {
            KeyAction::KeyCode(keycode) => {
                let code = keycode.value();

                if let Some(config) = self.tap_dances.get(&code) {
                    KeyResult::KeyPressed(KeyState {
                        code,
                        timeout: config.timeout,
                        timestamp: Instant::now(),
                        released: false,
                        tap: config.tap.clone(),
                        hold: config.hold.clone(),
                    })
                } else {
                    KeyResult::KeyCode(code)
                }
            }
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
            KeyAction::KeyCode(keycode) => {
                let value = keycode.value();

                if let Some(state) = self.pressed_keys.iter_mut().find(|s| s.code == value) {
                    state.released = true;
                    KeyResult::None
                } else {
                    KeyResult::KeyCode(value)
                }
            }
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
