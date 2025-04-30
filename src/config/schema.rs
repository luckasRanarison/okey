use std::{cell::RefCell, collections::HashMap, hash::Hash, str::FromStr};

use serde::{
    de::{value::StringDeserializer, IntoDeserializer},
    Deserialize, Deserializer,
};

use super::defaults;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: DefaultConfig,
    pub keyboards: Vec<KeyboardConfig>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DefaultConfig {
    #[serde(default)]
    pub tap_dance: DefaultTapDanceConfig,
    #[serde(default)]
    pub combo: DefaultComboConfig,
    #[serde(default)]
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
    #[serde(default = "defaults::event_poll_timeout")]
    pub event_poll_timeout: u16,
    #[serde(default = "defaults::deferred_key_delay")]
    pub deferred_key_delay: u16,
    #[serde(default = "defaults::unicode_input_delay")]
    pub unicode_input_delay: u16,
    #[serde(default = "defaults::maximum_lookup_depth")]
    pub maximum_lookup_depth: u8,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            event_poll_timeout: defaults::event_poll_timeout(),
            deferred_key_delay: defaults::deferred_key_delay(),
            unicode_input_delay: defaults::unicode_input_delay(),
            maximum_lookup_depth: defaults::maximum_lookup_depth(),
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
    pub layers: HashMap<String, LayerDefinition>,
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
    Macro(Macro),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Macro {
    Single(EventMacro),
    Sequence(Vec<EventMacro>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EventMacro {
    Tap(KeyCode),
    Press { press: KeyCode },
    Hold { hold: KeyCode },
    Release { release: KeyCode },
    Delay { delay: u32 },
    String { string: String },
    Env { env: String },
    Unicode { unicode: String },
    Shell { shell: String, trim: Option<bool> },
}

const SAFE_KEYCODE_START: u16 = 999;
const SHIFTED_KEYCODE_START: u16 = 800;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyCode(evdev::KeyCode);

impl From<evdev::KeyCode> for KeyCode {
    fn from(value: evdev::KeyCode) -> Self {
        Self(value)
    }
}

const fn to_shifted_code(code: evdev::KeyCode) -> isize {
    (SHIFTED_KEYCODE_START + code.code()) as isize
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum ShiftedKeycodes {
    KeyExclamation = to_shifted_code(evdev::KeyCode::KEY_1),
    KeyAt = to_shifted_code(evdev::KeyCode::KEY_2),
    KeyHash = to_shifted_code(evdev::KeyCode::KEY_3),
    KeyDollarsign = to_shifted_code(evdev::KeyCode::KEY_4),
    KeyPercent = to_shifted_code(evdev::KeyCode::KEY_5),
    KeyCaret = to_shifted_code(evdev::KeyCode::KEY_6),
    KeyAmpersand = to_shifted_code(evdev::KeyCode::KEY_7),
    KeyStar = to_shifted_code(evdev::KeyCode::KEY_8),
    KeyLeftparen = to_shifted_code(evdev::KeyCode::KEY_9),
    KeyRightparen = to_shifted_code(evdev::KeyCode::KEY_0),
    KeyUnderscore = to_shifted_code(evdev::KeyCode::KEY_MINUS),
    KeyPlus = to_shifted_code(evdev::KeyCode::KEY_EQUAL),
    KeyLeftcurly = to_shifted_code(evdev::KeyCode::KEY_LEFTBRACE),
    KeyRightcurly = to_shifted_code(evdev::KeyCode::KEY_RIGHTBRACE),
    KeyColon = to_shifted_code(evdev::KeyCode::KEY_SEMICOLON),
    KeyDoublequote = to_shifted_code(evdev::KeyCode::KEY_APOSTROPHE),
    KeyLess = to_shifted_code(evdev::KeyCode::KEY_COMMA),
    KeyGreater = to_shifted_code(evdev::KeyCode::KEY_DOT),
    KeyQuestion = to_shifted_code(evdev::KeyCode::KEY_SLASH),
    KeyTilde = to_shifted_code(evdev::KeyCode::KEY_GRAVE),
    KeyPipe = to_shifted_code(evdev::KeyCode::KEY_BACKSLASH),
}

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

    pub fn is_shifted(self) -> bool {
        self.0.code() > SHIFTED_KEYCODE_START && self.0.code() < SAFE_KEYCODE_START
    }

    pub fn shift() -> Self {
        KeyCode::new(evdev::KeyCode::KEY_LEFTSHIFT.code())
    }

    pub fn unshift(self) -> Self {
        KeyCode::new(self.0.code() - SHIFTED_KEYCODE_START)
    }
}

thread_local! {
    static CUSTOM_KEYCODES: RefCell<HashMap<String, u16>> = HashMap::default().into();
}

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key = String::deserialize(deserializer)?;
        let key_deserializer: StringDeserializer<D::Error> = key.clone().into_deserializer();

        if let Ok(shifted) = ShiftedKeycodes::deserialize(key_deserializer) {
            return Ok(KeyCode(evdev::KeyCode(shifted as u16)));
        }

        if let Ok(value) = evdev::KeyCode::from_str(&key) {
            return Ok(KeyCode(value));
        }

        CUSTOM_KEYCODES.with_borrow_mut(|keycodes| {
            let count = keycodes.len() as u16;
            let entry = keycodes.entry(key).or_insert(count + SAFE_KEYCODE_START);
            Ok(KeyCode(evdev::KeyCode::new(*entry)))
        })
    }
}
