use std::time::Instant;

use crate::config::schema::KeyAction;

#[derive(Debug)]
pub struct PressedKey {
    pub code: u16,
    pub timeout: u16,
    pub timestamp: Instant,
    pub released: bool,
    pub result: PressedKeyResult,
}

#[derive(Debug)]
pub enum PressedKeyResult {
    TapDance { tap: KeyAction, hold: KeyAction },
    Combo, // TODO
}

#[derive(Debug)]
pub enum KeyResult {
    KeyAction(KeyAction),
    KeyPressed(PressedKey),
    Layer,
    None,
}
