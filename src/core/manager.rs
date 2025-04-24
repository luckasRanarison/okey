use anyhow::Result;
use evdev::InputEvent;

use crate::config::schema::{
    DefaultConfig, KeyAction, KeyCode, KeyboardConfig, SAFE_KEYCODE_START,
};

use super::{
    combo::ComboManager,
    event::{
        EventEmitter, HOLD_EVENT, IntoInputEvent, IntoInputEvents, PRESS_EVENT, RELEASE_EVENT,
    },
    mapping::MappingManager,
    tap_dance::TapDanceManager,
};

#[derive(Debug)]
pub enum InputResult {
    Press(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Macro(Vec<KeyCode>),
    DoubleSequence(Box<[InputResult; 2]>),
    None,
}

#[derive(Debug)]
pub struct KeyManager {
    mapping_manager: MappingManager,
    combo_manager: ComboManager,
    tap_dance_manager: TapDanceManager,
    // layer_manager: LayerManager,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig, general: DefaultConfig) -> Self {
        let mapping_manager = MappingManager::new(config.keys);
        let combo_manager = ComboManager::new(config.combos, general.combo);
        let tap_dance_manager = TapDanceManager::new(config.tap_dances, general.tap_dance);
        // let layer_manager = LayerManager::new(config.layers);

        Self {
            mapping_manager,
            tap_dance_manager,
            combo_manager,
            // layer_manager,
        }
    }

    pub fn process_event<E: EventEmitter>(
        &mut self,
        event: InputEvent,
        emitter: &mut E,
    ) -> Result<()> {
        let code = self.mapping_manager.map(&event.code());
        // let code = self.layer_manager.map(&code);
        let value = event.value();

        let result = match value {
            PRESS_EVENT => self.handle_press(code),
            HOLD_EVENT => self.handle_hold(code),
            RELEASE_EVENT => self.handle_release(code),
            _ => unreachable!(),
        };

        self.dispatch_result(&result, emitter)
    }

    pub fn post_process<E: EventEmitter>(&mut self, emitter: &mut E) -> Result<()> {
        if let Some(results) = self.tap_dance_manager.process() {
            for result in results {
                self.dispatch_result(&result, emitter)?;
            }
        }

        if let Some(results) = self.combo_manager.process() {
            for result in results {
                self.dispatch_result(&result, emitter)?;
            }
        }

        Ok(())
    }

    fn dispatch_result<E: EventEmitter>(
        &mut self,
        result: &InputResult,
        emitter: &mut E,
    ) -> Result<()> {
        match result {
            InputResult::Press(code) | InputResult::Hold(code) | InputResult::Release(code) => {
                self.dispatch_event_result(result, code.clone(), emitter)
            }
            InputResult::DoubleSequence(results) => {
                let [first, second] = results.as_ref();
                self.dispatch_result(first, emitter)?;
                self.dispatch_result(second, emitter)
            }
            InputResult::Macro(codes) => emitter.emit(&codes.to_events()),
            InputResult::None => Ok(()),
        }
    }

    fn dispatch_event_result<E: EventEmitter>(
        &mut self,
        result: &InputResult,
        code: KeyCode,
        emitter: &mut E,
    ) -> Result<()> {
        let (event_kind, handler): (_, fn(&mut Self, KeyAction) -> InputResult) = match result {
            InputResult::Press(_) => (PRESS_EVENT, Self::handle_press),
            InputResult::Hold(_) => (HOLD_EVENT, Self::handle_hold),
            InputResult::Release(_) => (RELEASE_EVENT, Self::handle_release),
            _ => unreachable!(),
        };

        if code.value() >= SAFE_KEYCODE_START {
            let action = KeyAction::KeyCode(code);
            let result = handler(self, action);
            self.dispatch_result(&result, emitter)
        } else {
            emitter.emit(&[code.to_event(event_kind)])
        }
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
            KeyAction::KeyCode(code) => {
                let value = code.value();

                self.tap_dance_manager
                    .handle_hold(value)
                    .or_else(|| self.combo_manager.handle_hold(value))
                    .unwrap_or(InputResult::Hold(code))
            }
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
