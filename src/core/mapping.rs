use std::collections::HashMap;

use crate::config::schema::{KeyAction, KeyCode};

use super::shared::RawKeyCode;

#[derive(Debug)]
pub struct MappingManager {
    mappings: HashMap<RawKeyCode, KeyAction>,
}

impl MappingManager {
    pub fn new(mappings: HashMap<KeyCode, KeyAction>) -> Self {
        Self {
            mappings: mappings
                .into_iter()
                .map(|(key, value)| (key.value(), value))
                .collect(),
        }
    }

    pub fn map(&self, code: &RawKeyCode) -> KeyAction {
        self.mappings
            .get(code)
            .cloned()
            .unwrap_or(KeyAction::KeyCode(KeyCode::new(*code)))
    }
}
