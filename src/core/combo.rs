use std::{collections::HashSet, time::Instant};

use smallvec::SmallVec;

use crate::{
    config::schema::{ComboConfig, ComboDefinition, DefaultComboConfig, KeyAction, KeyCode},
    core::buffer::InputBuffer,
};

use super::{adapter::InputResult, shared::RawKeyCode};

#[derive(Debug)]
pub struct ComboManager {
    key_set: HashSet<RawKeyCode>,
    definitions: Vec<ComboDefinition>,
    pressed_keys: SmallVec<[ComboKey; 6]>,
    supressed_keys: SmallVec<[RawKeyCode; 6]>,
    active_combos: SmallVec<[ActiveCombo; 3]>,
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
            pressed_keys: SmallVec::default(),
            supressed_keys: SmallVec::default(),
            active_combos: SmallVec::default(),
        }
    }

    pub fn handle_press(&mut self, code: RawKeyCode) -> Option<InputResult> {
        if self.key_set.contains(&code) {
            self.pressed_keys.push(ComboKey::new(code));
            Some(InputResult::Pending(KeyCode::new(code)))
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: RawKeyCode) -> Option<InputResult> {
        let key = self.pressed_keys.iter_mut().find(|key| key.code == code);

        if let Some(key) = key {
            let now = Instant::now();
            let elapsed = now.duration_since(key.timestamp).as_millis();

            if elapsed > self.config.default_threshold as u128 {
                key.hold = true;
                return Some(InputResult::None);
            }
        }

        None
    }

    pub fn handle_release(&mut self, code: RawKeyCode) -> Option<InputResult> {
        let key = self.pressed_keys.iter_mut().find(|key| key.code == code);

        if let Some(key) = key {
            key.released = true;
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn process(&mut self, buffer: &mut InputBuffer) {
        if self.definitions.is_empty() {
            return;
        }

        self.process_key_results(buffer); // keys that exceeded threshold
        self.process_active_combos(buffer);
        self.process_combo_trigger(buffer);
    }

    fn process_key_results(&mut self, buffer: &mut InputBuffer) {
        let now = Instant::now();
        let threshold = self.config.default_threshold;

        for key in &self.pressed_keys {
            if key.released {
                buffer.push_key(key.code);
            }

            if self.supressed_keys.contains(&key.code) {
                continue;
            }

            if let Some(result) = key.get_key_result(key.code, now, threshold) {
                // Hold event, pass control back to the main handler
                if let InputResult::Press(_) = &result {
                    buffer.push_key(key.code);
                }

                buffer.push_result(result);
            }
        }

        while let Some(code) = buffer.pop_key() {
            self.pressed_keys.retain(|key| key.code != code);
        }
    }

    fn process_active_combos(&mut self, buffer: &mut InputBuffer) {
        for combo in &self.active_combos {
            let pressed_key = self.get_pressed_combo_key(combo);

            if let Some(key) = pressed_key {
                if let Some(code) = combo.code {
                    if key.hold {
                        buffer.push_result(InputResult::Hold(KeyCode::new(code)));
                    }
                }
            } else {
                if let Some(code) = combo.code {
                    buffer.push_result(InputResult::Release(KeyCode::new(code)));
                }

                for key in &combo.keys {
                    self.supressed_keys.retain(|code| *code == key.value());
                }

                buffer.push_key(combo.id);
            }
        }

        while let Some(id) = buffer.pop_key() {
            self.active_combos.retain(|combo| combo.id != id);
        }
    }

    fn process_combo_trigger(&mut self, buffer: &mut InputBuffer) {
        for (id, combo) in self.definitions.iter().enumerate() {
            if !self.should_activate_combo(combo) || self.is_combo_supressed(combo) {
                continue;
            }

            self.supressed_keys
                .extend(combo.keys.iter().map(|key| key.value()));

            let (code, result) = match &combo.action {
                KeyAction::KeyCode(code) => (Some(code.value()), InputResult::Press(*code)),
                KeyAction::Macro(codes) => (None, InputResult::Macro(codes.clone())),
            };

            for key in &combo.keys {
                buffer.clear_pending_key(key);
            }

            self.active_combos
                .push(ActiveCombo::new(id as u16, code, combo.keys.clone()));

            buffer.push_result(result);
        }
    }

    fn get_pressed_combo_key(&self, combo: &ActiveCombo) -> Option<&ComboKey> {
        combo
            .keys
            .iter()
            .find_map(|key| self.pressed_keys.iter().find(|k| k.code == key.value()))
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
                .iter()
                .find(|k| k.code == key.value())
                .is_some_and(|value| !value.hold)
        })
    }
}

#[derive(Debug)]
struct ComboKey {
    code: RawKeyCode,
    timestamp: Instant,
    released: bool,
    hold: bool,
}

impl ComboKey {
    fn new(code: RawKeyCode) -> Self {
        ComboKey {
            code,
            timestamp: Instant::now(),
            released: false,
            hold: false,
        }
    }

    fn get_key_result(
        &self,
        code: RawKeyCode,
        now: Instant,
        threshold: u16,
    ) -> Option<InputResult> {
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
    id: u16,
    code: Option<RawKeyCode>,
    keys: Vec<KeyCode>,
}

impl ActiveCombo {
    fn new(id: u16, code: Option<RawKeyCode>, keys: Vec<KeyCode>) -> Self {
        Self { id, code, keys }
    }
}
