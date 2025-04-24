use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::config::schema::{ComboConfig, ComboDefinition, DefaultComboConfig, KeyAction, KeyCode};

use super::manager::InputResult;

#[derive(Debug)]
pub struct ComboManager {
    key_set: HashSet<u16>,
    definitions: Vec<ComboDefinition>,
    pressed_keys: HashMap<u16, ComboKey>,
    supressed_keys: HashSet<u16>,
    active_combos: Vec<ActiveCombo>,
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
            supressed_keys: HashSet::new(),
            active_combos: Vec::new(),
        }
    }

    pub fn handle_press(&mut self, code: u16) -> Option<InputResult> {
        if self.key_set.contains(&code) {
            self.pressed_keys.insert(code, ComboKey::new());
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: u16) -> Option<InputResult> {
        if let Some(key) = self.pressed_keys.get_mut(&code) {
            let now = Instant::now();
            let elapsed = now.duration_since(key.timestamp).as_millis();

            if elapsed > self.config.default_threshold as u128 {
                key.hold = true;
                return Some(InputResult::None);
            }
        }

        None
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
        let threshold = self.config.default_threshold;
        let mut processed = Vec::new();

        for (&code, key) in &self.pressed_keys {
            if key.released {
                processed.push(code);
            }

            if self.supressed_keys.contains(&code) {
                continue;
            }

            if let Some(result) = key.get_key_result(code, now, threshold) {
                // Hold event, pass control back to the main handler
                if let InputResult::Press(_) = &result {
                    processed.push(code);
                }

                results.push(result);
            }
        }

        for code in processed.iter().rev() {
            self.pressed_keys.remove(code);
        }
    }

    fn process_active_combos(&mut self, results: &mut Vec<InputResult>) {
        let mut processed_combos = Vec::new();

        for (idx, combo) in self.active_combos.iter().enumerate() {
            let pressed_key = self.get_pressed_combo_key(combo);

            if let Some((key, code)) = pressed_key.zip(combo.code) {
                if key.hold {
                    results.push(InputResult::Hold(KeyCode::new(code)));
                }
            } else {
                if let Some(code) = combo.code {
                    results.push(InputResult::Release(KeyCode::new(code)));
                }

                for key in &combo.keys {
                    self.supressed_keys.remove(&key.value());
                }

                processed_combos.push(idx);
            }
        }

        for &idx in processed_combos.iter().rev() {
            self.active_combos.remove(idx);
        }
    }

    fn process_combo_trigger(&mut self, results: &mut Vec<InputResult>) {
        for combo in &self.definitions {
            if !self.should_activate_combo(combo) || self.is_combo_supressed(combo) {
                continue;
            }

            self.supressed_keys
                .extend(combo.keys.iter().map(|key| key.value()));

            let (code, result) = match &combo.action {
                KeyAction::KeyCode(code) => (Some(code.value()), InputResult::Press(code.clone())),
                KeyAction::Macro(codes) => (None, InputResult::Macro(codes.clone())),
            };

            self.active_combos
                .push(ActiveCombo::new(code, combo.keys.clone()));

            results.push(result);
        }
    }

    fn get_pressed_combo_key(&self, combo: &ActiveCombo) -> Option<&ComboKey> {
        combo
            .keys
            .iter()
            .find_map(|key| self.pressed_keys.get(&key.value()))
    }

    fn is_combo_supressed(&self, combo: &ComboDefinition) -> bool {
        combo
            .keys
            .iter()
            .any(|k| self.supressed_keys.contains(&k.value()))
    }

    fn should_activate_combo(&self, combo: &ComboDefinition) -> bool {
        combo.keys.iter().all(|key| {
            self.pressed_keys
                .get(&key.value())
                .is_some_and(|value| !value.hold)
        })
    }
}

#[derive(Debug)]
struct ComboKey {
    timestamp: Instant,
    released: bool,
    hold: bool,
}

impl ComboKey {
    fn new() -> Self {
        ComboKey {
            timestamp: Instant::now(),
            released: false,
            hold: false,
        }
    }

    fn get_key_result(&self, code: u16, now: Instant, threshold: u16) -> Option<InputResult> {
        let elapsed = now.duration_since(self.timestamp).as_millis();

        if elapsed < threshold as u128 {
            return None;
        }

        if self.released {
            let result = if self.hold {
                InputResult::Release(KeyCode::new(code))
            } else {
                InputResult::DoubleSequence(Box::new([
                    InputResult::Press(KeyCode::new(code)),
                    InputResult::Release(KeyCode::new(code)),
                ]))
            };

            return Some(result);
        } else if self.hold {
            return Some(InputResult::Press(KeyCode::new(code)));
        };

        None
    }
}

#[derive(Debug)]
struct ActiveCombo {
    code: Option<u16>,
    keys: Vec<KeyCode>,
}

impl ActiveCombo {
    fn new(code: Option<u16>, keys: Vec<KeyCode>) -> Self {
        Self { code, keys }
    }
}
