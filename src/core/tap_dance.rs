use std::{collections::HashMap, time::Instant};

use crate::config::schema::{KeyAction, KeyCode, TapDanceConfig};

use super::manager::InputResult;

#[derive(Debug)]
pub struct TapDanceManager {
    tap_dances: HashMap<u16, TapDanceConfig>,
    pressed_keys: Vec<PressedKey>,
}

impl TapDanceManager {
    pub fn new(config: HashMap<KeyCode, TapDanceConfig>) -> Self {
        let tap_dances = config
            .into_iter()
            .map(|(key, value)| (key.value(), value))
            .collect();

        Self {
            tap_dances,
            pressed_keys: Vec::new(),
        }
    }

    pub fn handle_press(&mut self, code: u16) -> Option<InputResult> {
        if let Some(config) = self.tap_dances.get(&code) {
            self.pressed_keys.push(PressedKey::new(code, config));
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&self, code: u16) -> Option<InputResult> {
        let key = self.pressed_keys.iter().find(|s| s.code == code);

        if let Some(key) = key {
            match &key.hold {
                KeyAction::KeyCode(code) => Some(InputResult::KeyHold(code.clone())),
                KeyAction::Macro(codes) => Some(InputResult::KeyMacro(codes.clone())),
            }
        } else {
            None
        }
    }

    pub fn handle_release(&mut self, code: u16) -> Option<InputResult> {
        let key = self.pressed_keys.iter_mut().find(|s| s.code == code);

        if let Some(key) = key {
            key.released = true;
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn process(&mut self) -> Option<Vec<InputResult>> {
        if self.pressed_keys.is_empty() {
            return None;
        }

        let now = Instant::now();
        let mut results = Vec::new();
        let mut processed = Vec::new();

        for (idx, state) in self.pressed_keys.iter().enumerate() {
            let result = state.get_dance_result(now);

            results.push(result);

            if state.released {
                processed.push(idx);
            }
        }

        for &idx in processed.iter().rev() {
            self.pressed_keys.remove(idx);
        }

        Some(results)
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
    fn new(code: u16, config: &TapDanceConfig) -> Self {
        PressedKey {
            code,
            timeout: config.timeout,
            timestamp: Instant::now(),
            released: false,
            tap: config.tap.clone(),
            hold: config.hold.clone(),
        }
    }

    fn get_dance_result(&self, now: Instant) -> InputResult {
        let elapsed = now.duration_since(self.timestamp).as_millis();

        if !self.released {
            return InputResult::None;
        }

        if elapsed > self.timeout as u128 {
            match &self.hold {
                KeyAction::KeyCode(code) => InputResult::KeyRelease(code.clone()),
                KeyAction::Macro(_) => InputResult::None,
            }
        } else {
            match self.tap.clone() {
                KeyAction::KeyCode(code) => InputResult::KeyPress(code),
                KeyAction::Macro(codes) => InputResult::KeyMacro(codes),
            }
        }
    }
}
