use std::collections::HashMap;

use crate::config::schema::{KeyAction, KeyCode};

#[derive(Debug)]
pub struct KeyCodeMap(HashMap<u16, KeyAction>);

impl KeyCodeMap {
    pub fn new(value: HashMap<KeyCode, KeyAction>) -> Self {
        Self(
            value
                .into_iter()
                .map(|(key, value)| (key.value(), value))
                .collect(),
        )
    }

    pub fn get_mapped_keycode(&self, code: &u16) -> Option<u16> {
        self.0.get(code).and_then(|value| match value {
            KeyAction::KeyCode(keycode) => Some(keycode.value()),
            KeyAction::Macro(_) => None,
        })
    }
}
