use anyhow::Result;
use evdev::{InputEvent, uinput::VirtualDevice};

use crate::config::{
    schema::{KeyAction, KeyCode, KeyboardConfig},
    utils::KeyCodeMap,
};

use super::{
    combo::ComboManager,
    event::{HOLD_EVENT, IntoInputEvent, IntoInputEvents, PRESS_EVENT, RELEASE_EVENT},
    tap_dance::TapDanceManager,
};

#[derive(Debug)]
pub enum InputResult {
    Press(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Macro(Vec<KeyCode>),
    DoubleSequence { code: KeyCode, events: [i32; 2] },
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
            PRESS_EVENT => self.handle_press(code),
            HOLD_EVENT => self.handle_hold(code),
            RELEASE_EVENT => self.handle_release(code),
            _ => unreachable!(),
        };

        self.dispatch_result(result, virtual_device)
    }

    pub fn post_process(&mut self, virtual_device: &mut VirtualDevice) -> Result<()> {
        if let Some(results) = self.tap_dance_manager.process() {
            for result in results {
                self.dispatch_result(result, virtual_device)?;
            }
        }

        if let Some(results) = self.combo_manager.process() {
            for result in results {
                self.dispatch_result(result, virtual_device)?;
            }
        }

        Ok(())
    }

    fn dispatch_result(
        &self,
        result: InputResult,
        virtual_device: &mut VirtualDevice,
    ) -> Result<()> {
        match result {
            InputResult::Press(code) => {
                virtual_device.emit(&[code.to_event(PRESS_EVENT)])?;
            }
            InputResult::Hold(code) => {
                virtual_device.emit(&[code.to_event(HOLD_EVENT)])?;
            }
            InputResult::Release(code) => {
                virtual_device.emit(&[code.to_event(RELEASE_EVENT)])?;
            }
            InputResult::Macro(codes) => {
                virtual_device.emit(&codes.to_events())?;
            }
            InputResult::DoubleSequence { code, events } => {
                virtual_device.emit(&events.map(|e| code.to_event(e)))?;
            }
            InputResult::None => {}
        }

        Ok(())
    }

    fn handle_press(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                self.tap_dance_manager
                    .handle_press(value)
                    .or_else(|| self.combo_manager.handle_press(value))
                    .unwrap_or(InputResult::Press(code))
            }
            KeyAction::Macro(codes) => InputResult::Macro(codes),
        }
    }

    fn handle_hold(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => InputResult::Hold(code),
            KeyAction::Macro(_) => InputResult::None,
        }
    }

    fn handle_release(&mut self, action: KeyAction) -> InputResult {
        match action {
            KeyAction::KeyCode(code) => {
                let value = code.value();

                self.tap_dance_manager
                    .handle_release(value)
                    .or_else(|| self.combo_manager.handle_release(value))
                    .unwrap_or(InputResult::Release(code))
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
