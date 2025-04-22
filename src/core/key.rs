use evdev::{EventType, InputEvent};

use crate::config::schema::KeyCode;

#[derive(Debug)]
pub enum KeyResult {
    KeyCode(u16),
    KeyMacro(KeyMacro),
    Layer,
    None,
}

#[derive(Debug)]
pub struct KeyMacro(Vec<u16>);

impl KeyMacro {
    pub fn from_keycodes(keycodes: &[KeyCode]) -> Self {
        Self(keycodes.iter().map(|code| code.value()).collect())
    }

    pub fn into_events(self) -> Vec<InputEvent> {
        self.0
            .into_iter()
            .flat_map(|value| {
                [
                    InputEvent::new(EventType::KEY.0, value, 1),
                    InputEvent::new(EventType::KEY.0, value, 0),
                ]
            })
            .collect()
    }
}
