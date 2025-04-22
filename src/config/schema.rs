use std::{cell::RefCell, collections::HashMap, hash::Hash, str::FromStr};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboards: Vec<KeyboardConfig>,
}

#[derive(Debug, Deserialize)]
pub struct KeyboardConfig {
    pub name: String,
    pub keys: Option<HashMap<KeyCode, KeyAction>>,
    pub combos: Option<ComboConfig>,
    pub tap_dances: Option<HashMap<KeyCode, TapDanceConfig>>,
    pub layers: Option<LayerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ComboConfig(pub Vec<ComboDefinition>);

#[derive(Debug, Deserialize)]
pub struct ComboDefinition {
    pub keys: Vec<KeyCode>,
    pub action: KeyAction,
}

#[derive(Debug, Deserialize)]
pub struct TapDanceConfig {
    pub tap: KeyAction,
    pub hold: KeyAction,
}

#[derive(Debug, Deserialize)]
pub struct LayerConfig(pub HashMap<String, LayerDefinition>);

#[derive(Debug, Deserialize)]
pub struct LayerDefinition {
    pub modifier: LayerModiferConfig,
    pub keys: HashMap<KeyCode, KeyAction>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LayerModiferConfig {
    Simple(KeyCode),
    Custom {
        key: KeyCode,
        #[serde(default)]
        r#type: LayerModifierKind,
    },
}

impl LayerModiferConfig {
    pub fn get_modifer(&self) -> &KeyCode {
        match self {
            LayerModiferConfig::Simple(keycode) => keycode,
            LayerModiferConfig::Custom { key, r#type: _ } => key,
        }
    }

    pub fn get_modifer_kind(&self) -> LayerModifierKind {
        match self {
            LayerModiferConfig::Simple(_) => LayerModifierKind::Momentary,
            LayerModiferConfig::Custom { key: _, r#type } => *r#type,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerModifierKind {
    #[default]
    Momentary,
    Toggle,
    Oneshoot,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KeyAction {
    KeyCode(KeyCode),
    Macro(Vec<KeyCode>),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum KeyCode {
    Evdev(evdev::KeyCode),
    Custom(u16),
}

impl KeyCode {
    pub fn value(&self) -> u16 {
        match self {
            KeyCode::Evdev(keycode) => keycode.code(),
            KeyCode::Custom(keycode) => *keycode,
        }
    }
}

const SAFE_RANGE: u16 = 999;

thread_local! {
    static CUSTOM_KEYCODES: RefCell<HashMap<String, u16>> = HashMap::default().into();
}

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key = String::deserialize(deserializer)?;

        match evdev::KeyCode::from_str(&key) {
            Ok(value) => Ok(KeyCode::Evdev(value)),
            _ => CUSTOM_KEYCODES.with_borrow_mut(|keycodes| {
                let count = keycodes.len() as u16;
                let entry = keycodes.entry(key).or_insert(count + SAFE_RANGE);
                Ok(KeyCode::Custom(*entry))
            }),
        }
    }
}
