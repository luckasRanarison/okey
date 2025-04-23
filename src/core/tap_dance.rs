use std::{collections::HashMap, time::Instant};

use anyhow::Result;

use crate::config::schema::{KeyAction, KeyCode, TapDanceConfig};

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

    pub fn handle_press(&mut self, code: u16) -> bool {
        if let Some(config) = self.tap_dances.get(&code) {
            self.pressed_keys.push(PressedKey::new(code, config));
            true
        } else {
            false
        }
    }

    pub fn handle_release(&mut self, code: u16) -> bool {
        let key = self.pressed_keys.iter_mut().find(|s| s.code == code);

        if let Some(key) = key {
            key.released = true;
            true
        } else {
            false
        }
    }

    pub fn process<F>(&mut self, mut hanle_action: F) -> Result<()>
    where
        F: FnMut(&KeyAction) -> Result<()>,
    {
        if !self.pressed_keys.is_empty() {
            let now = Instant::now();
            let mut processed = Vec::new();

            for (idx, state) in self.pressed_keys.iter().enumerate() {
                if let Some(action) = state.get_dance_action(now) {
                    hanle_action(action)?;
                    processed.push(idx);
                }
            }

            for &idx in processed.iter().rev() {
                self.pressed_keys.remove(idx);
            }
        }

        Ok(())
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

    fn get_dance_action(&self, now: Instant) -> Option<&KeyAction> {
        let elapsed = now.duration_since(self.timestamp).as_millis();

        if elapsed > self.timeout as u128 {
            Some(&self.hold)
        } else if self.released {
            Some(&self.tap)
        } else {
            None
        }
    }
}
