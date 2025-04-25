use anyhow::Result;
use evdev::{uinput::VirtualDevice, EventType, InputEvent};

use crate::config::schema::{EventMacro, KeyCode};

use super::manager::InputResult;

pub const PRESS_EVENT: i32 = 1;
pub const HOLD_EVENT: i32 = 2;
pub const RELEASE_EVENT: i32 = 0;

pub trait IntoInputEvent {
    fn to_event(&self, value: i32) -> InputEvent;
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

impl IntoInputResult for EventMacro {
    fn to_result(&self) -> InputResult {
        match self {
            EventMacro::Press { press } => InputResult::Press(*press),
            EventMacro::Hold { hold } => InputResult::Hold(*hold),
            EventMacro::Release { release } => InputResult::Release(*release),
            EventMacro::Delay { delay: sleep } => InputResult::Delay(*sleep),
            EventMacro::Tap(code) => InputResult::DoubleSequence(Box::new([
                InputResult::Press(*code),
                InputResult::Release(*code),
            ])),
        }
    }
}
