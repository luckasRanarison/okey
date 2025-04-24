use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::{
    config::schema::{DefaultTapDanceConfig, KeyAction, KeyCode, TapDanceConfig},
    core::buffer::InputBuffer,
};

use super::manager::InputResult;

#[derive(Debug)]
pub struct TapDanceManager {
    tap_dances: HashMap<u16, TapDanceConfig>,
    pressed_keys: Vec<PressedKey>,
    supressed_keys: HashSet<u16>,
    config: DefaultTapDanceConfig,
}

impl TapDanceManager {
    pub fn new(
        tap_dances: HashMap<KeyCode, TapDanceConfig>,
        config: DefaultTapDanceConfig,
    ) -> Self {
        let tap_dances = tap_dances
            .into_iter()
            .map(|(key, value)| (key.value(), value))
            .collect();

        Self {
            config,
            tap_dances,
            pressed_keys: Vec::with_capacity(5),
            supressed_keys: HashSet::with_capacity(3),
        }
    }

    pub fn handle_press(&mut self, code: u16) -> Option<InputResult> {
        if let Some(config) = self.tap_dances.get(&code) {
            self.pressed_keys
                .push(PressedKey::new(code, config, self.config.default_timeout));

            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: u16) -> Option<InputResult> {
        self.tap_dances
            .contains_key(&code)
            .then_some(InputResult::None)
    }

    pub fn handle_release(&mut self, code: u16) -> Option<InputResult> {
        let key = self.pressed_keys.iter_mut().find(|s| s.code == code);

        self.supressed_keys.remove(&code);

        if let Some(key) = key {
            key.released = true;
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn process(&mut self, buffer: &mut InputBuffer) {
        if self.pressed_keys.is_empty() {
            return;
        }

        let now = Instant::now();

        for (idx, state) in self.pressed_keys.iter().enumerate() {
            if self.supressed_keys.contains(&state.code) {
                continue;
            }

            let result = state.get_dance_result(now);

            if let InputResult::Macro(_) = &result {
                self.supressed_keys.insert(state.code);
            }

            buffer.push_result(result);

            if state.released {
                buffer.push_key(idx as u16);
            }
        }

        while let Some(idx) = buffer.pop_key() {
            self.pressed_keys.remove(idx as usize);
        }
    }
}

#[derive(Debug)]
struct PressedKey {
    code: u16,
    timeout: u16,
    timestamp: Instant,
    released: bool,
    tap: KeyAction,
    hold: KeyAction,
}

impl PressedKey {
    fn new(code: u16, config: &TapDanceConfig, default_timeout: u16) -> Self {
        PressedKey {
            code,
            timeout: config.timeout.unwrap_or(default_timeout),
            timestamp: Instant::now(),
            released: false,
            tap: config.tap.clone(),
            hold: config.hold.clone(),
        }
    }

    fn get_dance_result(&self, now: Instant) -> InputResult {
        let elapsed = now.duration_since(self.timestamp).as_millis();
        let timeout = self.timeout as u128;

        if self.released && elapsed > timeout {
            self.get_release_result()
        } else if self.released {
            self.get_tap_result()
        } else if elapsed > timeout {
            self.get_hold_resut()
        } else {
            InputResult::None
        }
    }

    fn get_release_result(&self) -> InputResult {
        match &self.hold {
            KeyAction::KeyCode(code) => InputResult::Release(code.clone()),
            KeyAction::Macro(_) => InputResult::None,
        }
    }

    fn get_tap_result(&self) -> InputResult {
        match &self.tap {
            KeyAction::KeyCode(code) => InputResult::DoubleSequence(Box::new([
                InputResult::Press(code.clone()),
                InputResult::Release(code.clone()),
            ])),
            KeyAction::Macro(codes) => InputResult::Macro(codes.clone()),
        }
    }

    fn get_hold_resut(&self) -> InputResult {
        match &self.hold {
            KeyAction::KeyCode(code) => InputResult::DoubleSequence(Box::new([
                InputResult::Press(code.clone()),
                InputResult::Hold(code.clone()),
            ])),
            KeyAction::Macro(codes) => InputResult::Macro(codes.clone()),
        }
    }
}
