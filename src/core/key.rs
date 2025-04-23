use std::time::Instant;

use crate::config::schema::{KeyAction, KeyCode};

#[derive(Debug)]
pub struct PressedKey {
    pub code: u16,
    pub timeout: u16,
    pub timestamp: Instant,
    pub released: bool,
    pub result: PressedKeyResult,
}

impl PressedKey {
    pub fn get_dance_action(&self, now: Instant) -> Option<&KeyAction> {
        let elapsed = now.duration_since(self.timestamp).as_millis();

        if elapsed > self.timeout as u128 {
            self.to_hold()
        } else if self.released {
            self.to_tap()
        } else {
            None
        }
    }

    fn to_tap(&self) -> Option<&KeyAction> {
        match &self.result {
            PressedKeyResult::TapDance { tap, .. } => Some(tap),
            PressedKeyResult::Combo => None,
        }
    }

    fn to_hold(&self) -> Option<&KeyAction> {
        match &self.result {
            PressedKeyResult::TapDance { hold, .. } => Some(hold),
            PressedKeyResult::Combo => None,
        }
    }
}

#[derive(Debug)]
pub enum PressedKeyResult {
    TapDance { tap: KeyAction, hold: KeyAction },
    Combo, // TODO
}

#[derive(Debug)]
pub enum KeyResult {
    KeyCode(KeyCode),
    KeyMacro(Vec<KeyCode>),
    None,
}
