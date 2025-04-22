use evdev::{EventType, InputEvent};

use crate::config::schema::{KeyAction, KeyCode};

pub trait IntoInputEvent {
    fn to_events(&self) -> Vec<InputEvent>;
}

impl IntoInputEvent for KeyCode {
    fn to_events(&self) -> Vec<InputEvent> {
        vec![
            InputEvent::new(EventType::KEY.0, self.value(), 1),
            InputEvent::new(EventType::KEY.0, self.value(), 0),
        ]
    }
}

impl IntoInputEvent for KeyAction {
    fn to_events(&self) -> Vec<InputEvent> {
        match self {
            KeyAction::KeyCode(code) => code.to_events(),
            KeyAction::Macro(codes) => codes
                .into_iter()
                .flat_map(|code| code.to_events())
                .collect(),
        }
    }
}
