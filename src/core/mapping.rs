use std::collections::HashMap;

use crate::config::schema::{KeyAction, KeyCode};

#[derive(Debug)]
pub struct MappingManager {
    mappings: HashMap<u16, KeyAction>,
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

    pub fn map(&self, code: &u16) -> KeyAction {
        self.mappings
            .get(code)
            .cloned()
            .unwrap_or(KeyAction::KeyCode(KeyCode::new(*code)))
    }
}
