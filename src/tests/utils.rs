use anyhow::Result;
use evdev::{EventType, InputEvent, KeyCode};

use crate::{
    config::schema::Config,
    core::{EventEmitter, KeyManager},
};

const CONFIG: &str = include_str!("./config/okey.yaml");

pub trait EventProcessor {
    fn process<E: EventEmitter>(&mut self, event: InputEvent, emitter: &mut E) -> Result<()>;
}

impl EventProcessor for KeyManager {
    fn process<E: EventEmitter>(&mut self, event: InputEvent, emitter: &mut E) -> Result<()> {
        self.process_event(event, emitter)?;
        self.post_process(emitter)
    }
}

#[derive(Debug, Default)]
pub struct FakeEventEmitter {
    queue: Vec<InputEvent>,
}

impl FakeEventEmitter {
    pub fn queue(&self) -> &[InputEvent] {
        &self.queue
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

impl EventEmitter for FakeEventEmitter {
    fn emit(&mut self, events: &[evdev::InputEvent]) -> anyhow::Result<()> {
        Ok(self.queue.extend(events))
    }
}

pub fn get_test_manager() -> KeyManager {
    let mut config: Config = serde_yaml::from_str(CONFIG).unwrap();
    let keyboard = config.keyboards.remove(0);
    let defaults = config.defaults.clone();

    KeyManager::new(keyboard, defaults)
}

pub fn release(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 0)
}

pub fn press(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 1)
}

pub fn hold(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 2)
}
