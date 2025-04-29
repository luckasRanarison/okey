use std::{thread, time::Duration};

use anyhow::Result;
use evdev::{Device, EventType, InputEvent};

use crate::config::schema::{
    DefaultConfig, GeneralConfig, KeyAction, KeyCode, KeyboardConfig, Macro,
};

use super::{
    buffer::InputBuffer,
    combo::ComboManager,
    event::{IntoInputEvent, ToInputResult, HOLD_EVENT, PRESS_EVENT, RELEASE_EVENT},
    layer::LayerManager,
    mapping::MappingManager,
    proxy::EventProxy,
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
pub struct KeyAdapter<'a, P: EventProxy> {
    proxy: &'a mut P,
    buffer: InputBuffer,
    config: GeneralConfig,
    mapping_manager: MappingManager,
    combo_manager: ComboManager,
    tap_dance_manager: TapDanceManager,
    layer_manager: LayerManager,
    depth: u8,
}

impl<'a, P: EventProxy> KeyAdapter<'a, P> {
    pub fn new(config: KeyboardConfig, defaults: DefaultConfig, proxy: &'a mut P) -> Self {
        let mapping_manager = MappingManager::new(config.keys);
        let combo_manager = ComboManager::new(config.combos, defaults.combo);
        let tap_dance_manager = TapDanceManager::new(config.tap_dances, defaults.tap_dance);
        let layer_manager = LayerManager::new(config.layers);

        Self {
            proxy,
            mapping_manager,
            tap_dance_manager,
            combo_manager,
            layer_manager,
            config: defaults.general,
            buffer: InputBuffer::default(),
            depth: 0,
        }
    }

    pub fn hook(&mut self, device: &mut Device) -> Result<()> {
        device.grab()?;
        device.set_nonblocking(true)?;

        loop {
            self.proxy.wait(self.config.event_poll_timeout)?;

            if let Ok(events) = device.fetch_events() {
                for event in events {
                    if event.event_type() == EventType::KEY {
                        self.process_event(event)?;
                    }
                }
            }

            self.post_process()?;
        }
    }

    pub fn process_event(&mut self, event: InputEvent) -> Result<()> {
        let action = self.mapping_manager.map(&event.code());
        let action = self.layer_manager.map(action);

        let result = match event.value() {
            PRESS_EVENT => self.handle_press(action),
            HOLD_EVENT => self.handle_hold(action),
            RELEASE_EVENT => self.handle_release(action),
            value => unreachable!("Unknown event value: {value}"),
        };

        self.dispatch_result(&result)
    }

    pub fn post_process(&mut self) -> Result<()> {
        self.tap_dance_manager.process(&mut self.buffer);
        self.combo_manager.process(&mut self.buffer);

        while let Some(result) = self.buffer.pop_result() {
            self.dispatch_result(&result)?;
        }

        Ok(())
    }

    fn dispatch_result(&mut self, result: &InputResult) -> Result<()> {
        if self.depth > self.config.maximum_lookup_depth {
            log::warn!("Maximum keycode lookup depth exceeded");
            return Ok(());
        }

        self.depth += 1;

        match result {
            InputResult::Pending(code) => {
                self.buffer.set_pending_key(*code);
            }

            InputResult::Release(code) if self.buffer.is_pending_key(code) => {
                self.dispatch_pending_key(code, result)?;
            }

            InputResult::Press(code)
                if !self.buffer.is_pending_key(code) && self.buffer.has_pending_keys() =>
            {
                self.buffer.defer_key(*code);
            }

            InputResult::Press(code) | InputResult::Hold(code) | InputResult::Release(code) => {
                self.dispatch_event_result(result, *code)?;
            }

            InputResult::DoubleSequence(results) => {
                let [first, second] = results.as_ref();
                self.dispatch_result(first)?;
                self.dispatch_result(second)?;
            }

            InputResult::Macro(value) => {
                self.dispatch_event_macro(value)?;
            }

            InputResult::Delay(timeout) => {
                thread::sleep(Duration::from_millis(*timeout as u64));
            }

            InputResult::None => {}
        }

        self.depth -= 1;

        Ok(())
    }

    fn dispatch_event_result(&mut self, result: &InputResult, code: KeyCode) -> Result<()> {
        let (event_kind, handler): (_, fn(&mut Self, KeyAction) -> InputResult) = match result {
            InputResult::Press(_) => (PRESS_EVENT, Self::handle_press),
            InputResult::Hold(_) => (HOLD_EVENT, Self::handle_hold),
            InputResult::Release(_) => (RELEASE_EVENT, Self::handle_release),
            value => unreachable!("Unexpected input result: {value:?}"),
        };

        let action = self.mapping_manager.map(&code.value());
        let action = self.layer_manager.map(action);

        match action {
            KeyAction::KeyCode(code) if !code.is_custom() => {
                self.proxy.emit(&[code.to_event(event_kind)])
            }
            _ => {
                let result = handler(self, action);
                self.dispatch_result(&result)
            }
        }
    }

    fn dispatch_pending_key(&mut self, code: &KeyCode, result: &InputResult) -> Result<()> {
        self.buffer.clear_pending_key(code);
        self.dispatch_event_result(result, *code)?;

        if !self.buffer.has_pending_keys() {
            while let Some(key) = self.buffer.pop_deferred_key() {
                self.proxy.wait(self.config.deferred_key_delay)?; // add a small delay to make input smoother

                let result = InputResult::DoubleSequence(Box::new([
                    InputResult::Press(key),
                    InputResult::Release(key),
                ]));

                self.dispatch_result(&result)?;
            }
        }

        Ok(())
    }

    fn dispatch_event_macro(&mut self, value: &Macro) -> Result<()> {
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
            self.dispatch_result(&event)?;
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
                    .or_else(|| self.layer_manager.handle_press(value))
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
                    .or_else(|| self.layer_manager.handle_hold(value))
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
                    .or_else(|| self.layer_manager.handle_release(value))
                    .unwrap_or(InputResult::Release(code))
            }
            _ => InputResult::None,
        }
    }
}
