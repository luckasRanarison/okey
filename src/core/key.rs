use std::time::Instant;

use evdev::{EventType, InputEvent};

use crate::config::schema::{KeyAction, KeyCode};

#[derive(Debug)]
pub struct KeyState {
    pub code: u16,
    pub timeout: u16,
    pub timestamp: Instant,
    pub released: bool,
    pub tap: KeyAction,
    pub hold: KeyAction,
}

#[derive(Debug)]
pub enum KeyResult {
    KeyCode(u16),
    KeyMacro(KeyMacro),
    KeyPressed(KeyState),
    Layer,
    None,
}

#[derive(Debug)]
pub struct KeyMacro(Vec<u16>);

impl KeyMacro {
    pub fn from_keycodes(keycodes: Vec<KeyCode>) -> Self {
        Self(
            keycodes
                .into_iter()
                .map(|keycode| keycode.value())
                .collect(),
        )
    }

    pub fn into_events(self) -> Vec<InputEvent> {
        self.0
            .into_iter()
            .flat_map(|value| {
                [
                    InputEvent::new(EventType::KEY.0, value, 1),
                    InputEvent::new(EventType::KEY.0, value, 0),
                ]
            })
            .collect()
    }
}
