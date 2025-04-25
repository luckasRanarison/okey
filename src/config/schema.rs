use std::{cell::RefCell, collections::HashMap, hash::Hash, str::FromStr};

use serde::{Deserialize, Deserializer};

use super::defaults;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: DefaultConfig,
    pub keyboards: Vec<KeyboardConfig>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DefaultConfig {
    pub tap_dance: DefaultTapDanceConfig,
    pub combo: DefaultComboConfig,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultTapDanceConfig {
    #[serde(default = "defaults::tap_dance_timeout")]
    pub default_timeout: u16,
}

impl Default for DefaultTapDanceConfig {
    fn default() -> Self {
        Self {
            default_timeout: defaults::tap_dance_timeout(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultComboConfig {
    #[serde(default = "defaults::combo_threshold")]
    pub default_threshold: u16,
}

impl Default for DefaultComboConfig {
    fn default() -> Self {
        Self {
            default_threshold: defaults::combo_threshold(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "defaults::deferred_key_delay")]
    pub deferred_key_delay: u16,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            deferred_key_delay: defaults::deferred_key_delay(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyboardConfig {
    pub name: String,
    #[serde(default)]
    pub keys: HashMap<KeyCode, KeyAction>,
    #[serde(default)]
    pub combos: ComboConfig,
    #[serde(default)]
    pub tap_dances: HashMap<KeyCode, TapDanceConfig>,
    #[serde(default)]
    pub layers: LayerConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ComboConfig(pub Vec<ComboDefinition>);

#[derive(Debug, Deserialize)]
pub struct ComboDefinition {
    pub keys: Vec<KeyCode>,
    pub action: KeyAction,
}

#[derive(Debug, Deserialize)]
pub struct TapDanceConfig {
    pub timeout: Option<u16>,
    pub tap: KeyAction,
    pub hold: KeyAction,
}

#[derive(Debug, Default, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum KeyAction {
    KeyCode(KeyCode),
    Macro(Vec<KeyCode>),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyCode(evdev::KeyCode);

impl KeyCode {
    pub fn new(code: u16) -> Self {
        Self(evdev::KeyCode::new(code))
    }

    pub fn value(self) -> u16 {
        self.0.code()
    }

    pub fn is_custom(self) -> bool {
        self.0.code() >= SAFE_KEYCODE_START
    }
}

pub const SAFE_KEYCODE_START: u16 = 999;

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
            Ok(value) => Ok(KeyCode(value)),
            _ => CUSTOM_KEYCODES.with_borrow_mut(|keycodes| {
                let count = keycodes.len() as u16;
                let entry = keycodes.entry(key).or_insert(count + SAFE_KEYCODE_START);
                Ok(KeyCode(evdev::KeyCode::new(*entry)))
            }),
        }
    }
}
