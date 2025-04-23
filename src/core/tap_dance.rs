use std::{collections::HashMap, time::Instant};

use crate::config::schema::{DefaultTapDanceConfig, KeyAction, KeyCode, TapDanceConfig};

use super::{
    event::{HOLD_EVENT, PRESS_EVENT, RELEASE_EVENT},
    manager::InputResult,
};

#[derive(Debug)]
pub struct TapDanceManager {
    tap_dances: HashMap<u16, TapDanceConfig>,
    pressed_keys: Vec<PressedKey>,
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
            pressed_keys: Vec::new(),
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

        for (idx, state) in self.pressed_keys.iter_mut().enumerate() {
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

    fn get_dance_result(&mut self, now: Instant) -> InputResult {
        let elapsed = now.duration_since(self.timestamp).as_millis();
        let timeout = self.timeout as u128;

        if self.released {
            if elapsed > timeout {
                match &self.hold {
                    KeyAction::KeyCode(code) => InputResult::Release(code.clone()),
                    KeyAction::Macro(_) => InputResult::None,
                }
            } else {
                match &self.tap {
                    KeyAction::KeyCode(code) => InputResult::DoubleSequence {
                        code: code.clone(),
                        events: [PRESS_EVENT, RELEASE_EVENT],
                    },
                    KeyAction::Macro(codes) => InputResult::Macro(codes.clone()),
                }
            }
        } else if elapsed > timeout {
            match &self.hold {
                KeyAction::KeyCode(code) => InputResult::DoubleSequence {
                    code: code.clone(),
                    events: [PRESS_EVENT, HOLD_EVENT],
                },
                KeyAction::Macro(codes) => {
                    self.released = true; // prevent macros from repeating
                    InputResult::Macro(codes.clone())
                }
            }
        } else {
            InputResult::None
        }
    }
}
