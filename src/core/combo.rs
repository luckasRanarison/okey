use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::config::schema::{ComboConfig, ComboDefinition, DefaultComboConfig, KeyAction, KeyCode};

use super::{
    event::{PRESS_EVENT, RELEASE_EVENT},
    manager::InputResult,
};

#[derive(Debug)]
pub struct ComboManager {
    key_set: HashSet<u16>,
    definitions: Vec<ComboDefinition>,
    pressed_keys: HashMap<u16, ComboKey>,
    active_combos: HashMap<u16, Vec<KeyCode>>,
    supressed_keys: HashSet<u16>,
    config: DefaultComboConfig,
}

impl ComboManager {
    pub fn new(combos: ComboConfig, config: DefaultComboConfig) -> Self {
        let key_set = combos
            .0
            .iter()
            .flat_map(|def| def.keys.iter().map(|key| key.value()))
            .collect();

        let mut definitions = combos.0;

        definitions.sort_by(|a, b| b.keys.len().cmp(&a.keys.len()));

        Self {
            config,
            key_set,
            definitions,
            pressed_keys: HashMap::new(),
            active_combos: HashMap::new(),
            supressed_keys: HashSet::new(),
        }
    }

    pub fn handle_press(&mut self, code: u16) -> Option<InputResult> {
        if self.key_set.contains(&code) {
            self.pressed_keys
                .insert(code, ComboKey::new(self.config.default_threshold));

            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: u16) -> Option<InputResult> {
        if let Some(key) = self.pressed_keys.get_mut(&code) {
            key.hold = true;
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_release(&mut self, code: u16) -> Option<InputResult> {
        if let Some(key) = self.pressed_keys.get_mut(&code) {
            key.released = true;
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn process(&mut self) -> Option<Vec<InputResult>> {
        if self.definitions.is_empty() {
            return None;
        }

        let mut results = Vec::new();

        self.process_key_results(&mut results); // keys that exceeded threshold
        self.process_active_combos(&mut results);
        self.process_combo_trigger(&mut results);

        Some(results)
    }

    fn process_key_results(&mut self, results: &mut Vec<InputResult>) {
        let now = Instant::now();
        let mut processed = Vec::new();

        for (&code, key) in &self.pressed_keys {
            if key.released {
                processed.push(code);
            }

            if self.supressed_keys.contains(&code) {
                continue;
            }

            if let Some(result) = key.get_key_result(code, now) {
                results.push(result);
            }
        }

        for code in processed.iter().rev() {
            self.pressed_keys.remove(code);
        }
    }

    fn process_active_combos(&mut self, results: &mut Vec<InputResult>) {
        let mut processed_combos = Vec::new();

        for (&code, keys) in &self.active_combos {
            if let Some(key) = keys
                .iter()
                .find_map(|key| self.pressed_keys.get(&key.value()))
            {
                if key.hold {
                    results.push(InputResult::Hold(KeyCode::new(code)));
                }
            } else {
                results.push(InputResult::Release(KeyCode::new(code)));
                processed_combos.push(code);

                for key in keys {
                    self.supressed_keys.remove(&key.value());
                }
            }
        }

        for code in processed_combos {
            self.active_combos.remove(&code);
        }
    }

    fn process_combo_trigger(&mut self, results: &mut Vec<InputResult>) {
        for combo in &self.definitions {
            let active = combo.keys.iter().all(|key| {
                self.pressed_keys
                    .get(&key.value())
                    .is_some_and(|value| !value.hold)
            });

            if active {
                self.supressed_keys
                    .extend(combo.keys.iter().map(|key| key.value()));

                let result = match &combo.action {
                    KeyAction::KeyCode(code) => {
                        self.active_combos.insert(code.value(), combo.keys.clone());

                        InputResult::Press(code.clone())
                    }
                    KeyAction::Macro(codes) => InputResult::Macro(codes.clone()),
                };

                results.push(result);
            }
        }
    }
}

#[derive(Debug)]
struct ComboKey {
    timeout: u16,
    timestamp: Instant,
    released: bool,
    hold: bool,
}

impl ComboKey {
    fn new(threshold: u16) -> Self {
        ComboKey {
            timeout: threshold,
            timestamp: Instant::now(),
            released: false,
            hold: false,
        }
    }

    fn get_key_result(&self, code: u16, now: Instant) -> Option<InputResult> {
        let elapsed = now.duration_since(self.timestamp).as_millis();

        if elapsed < self.timeout as u128 {
            return None;
        }

        if self.released {
            let result = if self.hold {
                InputResult::Release(KeyCode::new(code))
            } else {
                InputResult::DoubleSequence {
                    code: KeyCode::new(code),
                    events: [PRESS_EVENT, RELEASE_EVENT],
                }
            };

            return Some(result);
        } else if self.hold {
            return Some(InputResult::Press(KeyCode::new(code)));
        };

        None
    }
}
