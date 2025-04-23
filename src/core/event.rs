use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::schema::{KeyAction, KeyCode};

pub const PRESS_EVENT: i32 = 1;
pub const HOLD_EVENT: i32 = 2;
pub const RELEASE_EVENT: i32 = 0;

pub trait IntoInputEvent {
    fn to_event(&self, value: i32) -> InputEvent;
}

pub trait IntoInputEvents {
    fn to_events(&self) -> Vec<InputEvent>;
}

pub trait EventEmitter {
    fn emit(&mut self, events: &[InputEvent]) -> Result<()>;
}

impl EventEmitter for VirtualDevice {
    fn emit(&mut self, events: &[InputEvent]) -> Result<()> {
        Ok(self.emit(events)?)
    }
}

impl IntoInputEvent for KeyCode {
    fn to_event(&self, value: i32) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.value(), value)
    }
}

impl IntoInputEvents for KeyCode {
    fn to_events(&self) -> Vec<InputEvent> {
        vec![
            InputEvent::new(EventType::KEY.0, self.value(), 1),
            InputEvent::new(EventType::KEY.0, self.value(), 0),
        ]
    }
}

impl IntoInputEvents for Vec<KeyCode> {
    fn to_events(&self) -> Vec<InputEvent> {
        self.iter().flat_map(|code| code.to_events()).collect()
    }
}

impl IntoInputEvents for KeyAction {
    fn to_events(&self) -> Vec<InputEvent> {
        match self {
            KeyAction::KeyCode(code) => code.to_events(),
            KeyAction::Macro(codes) => codes.to_events(),
        }
    }
}
