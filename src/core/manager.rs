use std::{thread, time::Duration};

use anyhow::Result;
use evdev::InputEvent;

use crate::config::schema::{
    DefaultConfig, GeneralConfig, KeyAction, KeyCode, KeyboardConfig, Macro,
};

use super::{
    buffer::InputBuffer,
    combo::ComboManager,
    event::{EventEmitter, HOLD_EVENT, IntoInputEvent, PRESS_EVENT, RELEASE_EVENT, ToInputResult},
    mapping::MappingManager,
    tap_dance::TapDanceManager,
};

#[derive(Debug)]
pub enum InputResult {
    Press(KeyCode),
    Pending(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Macro(Macro),
    DoubleSequence(Box<[InputResult; 2]>),
    Delay(u32),
    None,
}

#[derive(Debug)]
pub struct KeyManager {
    mapping_manager: MappingManager,
    combo_manager: ComboManager,
    tap_dance_manager: TapDanceManager,
    buffer: InputBuffer,
    config: GeneralConfig,
    // layer_manager: LayerManager,
}

impl KeyManager {
    pub fn new(config: KeyboardConfig, defaults: DefaultConfig) -> Self {
        let mapping_manager = MappingManager::new(config.keys);
        let combo_manager = ComboManager::new(config.combos, defaults.combo);
        let tap_dance_manager = TapDanceManager::new(config.tap_dances, defaults.tap_dance);
        // let layer_manager = LayerManager::new(config.layers);

        Self {
            mapping_manager,
            tap_dance_manager,
            combo_manager,
            config: defaults.general,
            buffer: InputBuffer::default(),
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
        self.tap_dance_manager.process(&mut self.buffer);
        self.combo_manager.process(&mut self.buffer);

        while let Some(result) = self.buffer.pop_result() {
            self.dispatch_result(&result, emitter)?;
        }

        Ok(())
    }

    fn dispatch_result<E: EventEmitter>(
        &mut self,
        result: &InputResult,
        emitter: &mut E,
    ) -> Result<()> {
        match result {
            InputResult::Pending(code) => {
                self.buffer.set_pending_key(*code);
            }

            InputResult::Release(code) if self.buffer.is_pending_key(code) => {
                self.buffer.clear_pending_key(code);
                self.dispatch_event_result(result, *code, emitter)?;

                if !self.buffer.has_pending_keys() {
                    self.dispatch_deferred_events(emitter)?;
                }
            }

            InputResult::Press(code)
                if !self.buffer.is_pending_key(code) && self.buffer.has_pending_keys() =>
            {
                self.buffer.defer_key(*code);
            }

            InputResult::Press(code) | InputResult::Hold(code) | InputResult::Release(code) => {
                self.dispatch_event_result(result, *code, emitter)?;
            }

            InputResult::DoubleSequence(results) => {
                let [first, second] = results.as_ref();
                self.dispatch_result(first, emitter)?;
                self.dispatch_result(second, emitter)?;
            }

            InputResult::Macro(value) => {
                self.dispatch_event_macro(value, emitter)?;
            }

            InputResult::Delay(timeout) => {
                thread::sleep(Duration::from_millis(*timeout as u64));
            }

            InputResult::None => {}
        }

        Ok(())
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

        if code.is_custom() {
            let action = KeyAction::KeyCode(code);
            let result = handler(self, action);
            self.dispatch_result(&result, emitter)
        } else {
            emitter.emit(&[code.to_event(event_kind)])
        }
    }

    fn dispatch_deferred_events<E: EventEmitter>(&mut self, emitter: &mut E) -> Result<()> {
        let delay = Duration::from_millis(self.config.deferred_key_delay.into());

        while let Some(key) = self.buffer.pop_deferred_key() {
            thread::sleep(delay); // add a small delay to make input smoother

            let result = InputResult::DoubleSequence(Box::new([
                InputResult::Press(key),
                InputResult::Release(key),
            ]));

            self.dispatch_result(&result, emitter)?;
        }

        Ok(())
    }

    fn dispatch_event_macro<E: EventEmitter>(
        &mut self,
        value: &Macro,
        emitter: &mut E,
    ) -> Result<()> {
        let delay = self.config.unicode_input_delay;

        let events = match value {
            Macro::Sequence(value) => value
                .iter()
                .map(|m| m.to_results(delay))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
            Macro::Single(event) => event.to_results(delay)?,
        };

        for event in events {
            self.dispatch_result(&event, emitter)?;
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
