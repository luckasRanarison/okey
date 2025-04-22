use std::collections::HashMap;

use crate::config::schema::{KeyAction, KeyCode};

use super::schema::{LayerConfig, LayerModifierKind};

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

    pub fn get(&self, code: &u16) -> Option<&KeyAction> {
        self.0.get(code)
    }

    pub fn get_keycode(&self, code: &u16) -> Option<u16> {
        self.0.get(code).and_then(|value| match value {
            KeyAction::KeyCode(keycode) => Some(keycode.value()),
            KeyAction::Macro(_) => None,
        })
    }

    pub fn map(&self, code: &u16) -> u16 {
        self.get_keycode(code).unwrap_or(*code)
    }
}

#[derive(Debug, Default)]
pub struct LayerMap(HashMap<u16, LayerEntry>);

impl LayerMap {
    pub fn get_entry(&self, keycode: &u16) -> Option<&LayerEntry> {
        self.0.get(keycode)
    }
}

impl From<LayerConfig> for LayerMap {
    fn from(value: LayerConfig) -> Self {
        let map = value
            .0
            .into_values()
            .map(|value| {
                (
                    value.modifier.get_modifer().value(),
                    LayerEntry {
                        kind: value.modifier.get_modifer_kind(),
                        keys: KeyCodeMap::new(value.keys),
                    },
                )
            })
            .collect();

        Self(map)
    }
}

#[derive(Debug)]
pub struct LayerEntry {
    pub kind: LayerModifierKind,
    pub keys: KeyCodeMap,
}
