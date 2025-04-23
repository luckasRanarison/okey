use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::config::schema::{ComboConfig, ComboDefinition, KeyAction};

use super::manager::InputResult;

#[derive(Debug)]
pub struct ComboManager {
    keys_set: HashSet<u16>,
    defintions: Vec<ComboDefinition>,
    pressed_keys: HashMap<u16, ComboKey>,
}

impl ComboManager {
    pub fn new(config: ComboConfig) -> Self {
        let key_set = config
            .0
            .iter()
            .flat_map(|def| def.keys.iter().map(|key| key.value()))
            .collect();

        let mut definitions = config.0;

        definitions.sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));

        Self {
            keys_set: key_set,
            defintions: definitions,
            pressed_keys: HashMap::new(),
        }
    }

    pub fn handle_press(&mut self, _code: u16) -> Option<InputResult> {
        None
    }

    pub fn handle_release(&mut self, _code: u16) -> Option<InputResult> {
        None
    }

    pub fn process(&mut self) -> Option<Vec<InputResult>> {
        if !self.defintions.is_empty() {
            //
        }

        None
    }
}

#[derive(Debug)]
struct ComboKey {
    timeout: u16,
    timestamp: Instant,
}
