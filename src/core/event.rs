use anyhow::Result;
use evdev::{EventType, InputEvent, uinput::VirtualDevice};

use crate::config::schema::{EventMacro, KeyCode, SimpleMacro};

use super::manager::InputResult;

pub const PRESS_EVENT: i32 = 1;
pub const HOLD_EVENT: i32 = 2;
pub const RELEASE_EVENT: i32 = 0;

pub trait IntoInputEvent {
    fn to_event(&self, value: i32) -> InputEvent;
}

pub trait IntoInputEvents {
    fn to_events(&self) -> Vec<InputEvent>;
}

pub trait IntoInputResult {
    fn to_result(&self) -> InputResult;
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

impl IntoInputEvents for SimpleMacro {
    fn to_events(&self) -> Vec<InputEvent> {
        self.0.iter().flat_map(|code| code.to_events()).collect()
    }
}

impl IntoInputResult for EventMacro {
    fn to_result(&self) -> InputResult {
        match self {
            EventMacro::Press { press } => InputResult::Press(*press),
            EventMacro::Hold { hold } => InputResult::Hold(*hold),
            EventMacro::Release { release } => InputResult::Release(*release),
            EventMacro::Sleep { sleep } => InputResult::Sleep(*sleep),
        }
    }
}
