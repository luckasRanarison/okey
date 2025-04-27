use anyhow::Result;
use evdev::{EventType, InputEvent};

use crate::config::schema::{EventMacro, KeyCode};

use super::{
    adapter::InputResult,
    input::{command_to_input, string_to_input, unicode_to_input},
};

pub const PRESS_EVENT: i32 = 1;
pub const HOLD_EVENT: i32 = 2;
pub const RELEASE_EVENT: i32 = 0;

pub trait IntoInputEvent {
    fn to_event(&self, value: i32) -> InputEvent;
}

pub trait ToInputResult {
    fn to_results(&self, delay: u16) -> Result<Vec<InputResult>>;
}

impl IntoInputEvent for KeyCode {
    fn to_event(&self, value: i32) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.value(), value)
    }
}

impl ToInputResult for EventMacro {
    fn to_results(&self, delay: u16) -> Result<Vec<InputResult>> {
        match self {
            EventMacro::Press { press } => Ok(vec![InputResult::Press(*press)]),
            EventMacro::Hold { hold } => Ok(vec![InputResult::Hold(*hold)]),
            EventMacro::Release { release } => Ok(vec![InputResult::Release(*release)]),
            EventMacro::Delay { delay: sleep } => Ok(vec![InputResult::Delay(*sleep)]),
            EventMacro::String { string } => string_to_input(string),
            EventMacro::Env { env } => string_to_input(&std::env::var(env)?),
            EventMacro::Unicode { unicode } => unicode_to_input(unicode, delay),
            EventMacro::Shell { shell, trim } => command_to_input(shell, *trim),
            EventMacro::Tap(code) => Ok(vec![InputResult::DoubleSequence(Box::new([
                InputResult::Press(*code),
                InputResult::Release(*code),
            ]))]),
        }
    }
}
