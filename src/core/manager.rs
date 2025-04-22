// use std::{collections::HashMap, time::Instant};

use std::time::Instant;

use anyhow::Result;
use evdev::{uinput::VirtualDevice, EventType, InputEvent};

use crate::config::{
    schema::{KeyAction, KeyboardConfig, LayerConfig},
    utils::{KeyCodeMap, LayerMap, TapDanceMap},
};

use super::{
    event::IntoInputEvent,
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
            KeyResult::KeyAction(action) => {
                virtual_device.emit(&action.to_events())?;
            }
            KeyResult::KeyPressed(state) => {
                self.pressed_keys.push(state);
            }

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
        let mut processed = Vec::new();

        for (idx, state) in self.pressed_keys.iter().enumerate() {
            let elapsed = now.duration_since(state.timestamp).as_millis();

            if elapsed > state.timeout as u128 {
                if let PressedKeyResult::TapDance { hold, .. } = &state.result {
                    virtual_device.emit(&hold.to_events())?;
                }

                processed.push(idx);
            } else if state.released {
                if let PressedKeyResult::TapDance { tap, .. } = &state.result {
                    virtual_device.emit(&tap.to_events())?;
                }

                processed.push(idx);
            }
        }

        for &idx in processed.iter().rev() {
            self.pressed_keys.remove(idx);
        }

        Ok(())
    }

    fn handle_press(&mut self, action: KeyAction) -> KeyResult {
        if let KeyAction::KeyCode(code) = &action {
            let code = code.value();

            if let Some(config) = self.tap_dances.get(&code) {
                return KeyResult::KeyPressed(PressedKey {
                    code,
                    timeout: config.timeout,
                    timestamp: Instant::now(),
                    released: false,
                    result: PressedKeyResult::TapDance {
                        tap: config.tap.clone(),
                        hold: config.hold.clone(),
                    },
                });
            }
        }

        KeyResult::KeyAction(action)
    }

    fn handle_hold(&mut self, action: KeyAction) -> KeyResult {
        match &action {
            KeyAction::KeyCode(_) => KeyResult::KeyAction(action),
            KeyAction::Macro(_) => KeyResult::None,
        }
    }

    fn handle_release(&mut self, action: KeyAction) -> KeyResult {
        if let KeyAction::KeyCode(keycode) = &action {
            let value = keycode.value();

            if let Some(state) = self.pressed_keys.iter_mut().find(|s| s.code == value) {
                state.released = true;
                KeyResult::None
            } else {
                KeyResult::KeyAction(action)
            }
        } else {
            KeyResult::None
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
