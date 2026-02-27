use std::{thread, time::Duration};

use evdev::{EventType, InputEvent};

use crate::{config::schema::Config, core::EventProxy};

pub use anyhow::Result;
pub use evdev::KeyCode;

pub use crate::core::KeyAdapter;

pub trait EventProcessor {
    fn process_input(&mut self, event: InputEvent) -> Result<()>;

    fn process_buffer(&mut self, buffer: &InputBuffer) -> Result<()> {
        for event in &buffer.value {
            self.process_input(*event)?;
        }

        Ok(())
    }

    fn process_sequence<I>(&mut self, sequence: I) -> Result<()>
    where
        I: IntoIterator<Item = InputSequence>,
    {
        for event in sequence.into_iter().flat_map(|ev| ev.into_events()) {
            self.process_input(event)?;
        }

        Ok(())
    }
}

impl<P: EventProxy> EventProcessor for KeyAdapter<'_, P> {
    fn process_input(&mut self, event: InputEvent) -> Result<()> {
        self.process_event(event)?;
        self.post_process()
    }
}

#[derive(Debug, Default)]
pub struct EventProxyMock {
    queue: Vec<InputEvent>,
}

impl EventProxyMock {
    pub fn queue(&self) -> &[InputEvent] {
        &self.queue
    }
}

impl EventProxy for EventProxyMock {
    fn emit(&mut self, events: &[evdev::InputEvent]) -> anyhow::Result<()> {
        self.queue.extend(events);
        Ok(())
    }

    fn wait(&mut self, timeout: u16) -> Result<()> {
        thread::sleep(Duration::from_millis(timeout.into()));
        Ok(())
    }
}

impl<'a, P: EventProxy> KeyAdapter<'a, P> {
    pub fn with_config(config: &str, proxy: &'a mut P) -> Self {
        let mut config: Config = serde_yaml::from_str(config).unwrap();
        let keyboard = config.keyboards.remove(0);
        let defaults = config.defaults.clone();

        KeyAdapter::new(keyboard, defaults, proxy)
    }
}

#[derive(Debug)]
pub struct InputBuffer {
    value: Vec<InputEvent>,
}

impl InputBuffer {
    pub fn new<I>(sequence: I) -> Self
    where
        I: IntoIterator<Item = InputSequence>,
    {
        Self {
            value: sequence
                .into_iter()
                .flat_map(|ev| ev.into_events())
                .collect(),
        }
    }

    pub fn value(&self) -> &[InputEvent] {
        &self.value
    }
}

#[derive(Debug)]
pub enum InputSequence {
    Press(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Shifted(KeyCode),
    Tap(KeyCode),
    TapHold(KeyCode),
    ComboPress(Vec<KeyCode>),
    ComboHold(Vec<KeyCode>),
    ComboRelease(Vec<KeyCode>),
    Unicode,
}

impl InputSequence {
    fn into_events(self) -> Vec<InputEvent> {
        match self {
            InputSequence::Press(code) => vec![press(code)],
            InputSequence::Hold(code) => vec![press(code), hold(code)],
            InputSequence::Release(code) => vec![release(code)],
            InputSequence::Tap(code) => vec![press(code), release(code)],
            InputSequence::TapHold(code) => vec![press(code), hold(code), release(code)],
            InputSequence::Shifted(code) => Self::shifted(code),
            InputSequence::ComboPress(keys) => keys.into_iter().map(|k| press(k)).collect(),
            InputSequence::ComboHold(keys) => keys.into_iter().map(|k| hold(k)).collect(),
            InputSequence::ComboRelease(keys) => keys.into_iter().map(|k| release(k)).collect(),
            InputSequence::Unicode => Self::unicode(),
        }
    }

    fn shifted(code: KeyCode) -> Vec<InputEvent> {
        vec![
            press(KeyCode::KEY_LEFTSHIFT),
            hold(KeyCode::KEY_LEFTSHIFT),
            press(code),
            release(code),
            release(KeyCode::KEY_LEFTSHIFT),
        ]
    }

    fn unicode() -> Vec<InputEvent> {
        vec![
            press(KeyCode::KEY_LEFTCTRL),
            press(KeyCode::KEY_LEFTSHIFT),
            press(KeyCode::KEY_U),
            hold(KeyCode::KEY_LEFTCTRL),
            hold(KeyCode::KEY_LEFTSHIFT),
            hold(KeyCode::KEY_U),
            release(KeyCode::KEY_LEFTCTRL),
            release(KeyCode::KEY_LEFTSHIFT),
            release(KeyCode::KEY_U),
        ]
    }
}

fn release(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 0)
}

fn press(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 1)
}

fn hold(code: KeyCode) -> InputEvent {
    InputEvent::new(EventType::KEY.0, code.code(), 2)
}
