// use std::{collections::HashMap, time::Instant};

use anyhow::Result;
use evdev::{InputEvent, uinput::VirtualDevice};

use crate::config::{
    schema::{KeyAction, KeyCode, KeyboardConfig},
    utils::KeyCodeMap,
};

use super::{
    combo::ComboManager,
    event::{IntoInputEvent, IntoInputEvents},
    tap_dance::TapDanceManager,
};

#[derive(Debug)]
enum InputResult {
    KeyCode(KeyCode),
    KeyMacro(Vec<KeyCode>),
    None,
}

#[derive(Debug)]
pub struct KeyManager {
    mappings: KeyCodeMap,
    tap_dance_manager: TapDanceManager,
    combo_manager: ComboManager,
    // layer_manager: LayerManager,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig) -> Self {
        let mappings = KeyCodeMap::new(config.keys.unwrap_or_default());
        let tap_dance_manager = TapDanceManager::new(config.tap_dances.unwrap_or_default());
        let combo_manager = ComboManager::new(config.combos.unwrap_or_default());
        // let layer_manager = LayerManager::new(config.layers);

        Self {
            mappings,
            tap_dance_manager,
            combo_manager,
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
        self.tap_dance_manager
            .process(|action| Ok(virtual_device.emit(&action.to_events())?))?;

        self.combo_manager
            .process(|action| Ok(virtual_device.emit(&action.to_events())?))?;

        Ok(())
    }

    fn dispatch_result(
        &mut self,
        result: InputResult,
        event_value: i32,
        virtual_device: &mut VirtualDevice,
    ) -> Result<()> {
        match result {
            InputResult::KeyCode(code) => {
                virtual_device.emit(&[code.to_event(event_value)])?;
            }
            InputResult::KeyMacro(codes) => {
                virtual_device.emit(&codes.to_events())?;
            }
            InputResult::None => {}
        }

        Ok(())
    }

    fn handle_press(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                if self.tap_dance_manager.handle_press(value)
                    || self.combo_manager.handle_press(value)
                {
                    InputResult::None
                } else {
                    InputResult::KeyCode(code)
                }
            }
            KeyAction::Macro(codes) => InputResult::KeyMacro(codes),
        }
    }

    fn handle_hold(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => InputResult::KeyCode(code),
            KeyAction::Macro(_) => InputResult::None,
        }
    }

    fn handle_release(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                if self.tap_dance_manager.handle_release(value)
                    || self.combo_manager.handle_release(value)
                {
                    InputResult::None
                } else {
                    InputResult::KeyCode(code)
                }
            }
            _ => InputResult::None,
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
