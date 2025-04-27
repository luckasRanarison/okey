use std::{thread, time::Duration};

use evdev::{EventType, InputEvent};

use crate::{config::schema::Config, core::EventProxy};

pub trait EventProcessor {
    fn process_input(&mut self, event: InputEvent) -> Result<()>;

    fn process(&mut self, buffer: InputBuffer) -> Result<()> {
        for event in buffer.value() {
            self.process_input(*event)?;
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

    pub fn clear(&mut self) {
        self.queue.clear();
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

pub trait EventTarget: Sized {
    fn code(&self) -> KeyCode;
    fn release(self) -> InputBuffer;
    fn press(self) -> InputBuffer;
    fn hold(self) -> InputBuffer;
    fn into_value(self) -> Vec<InputEvent>;

    fn tap(self) -> InputBuffer {
        self.press().release()
    }

    fn tap_hold(self) -> InputBuffer {
        self.press().hold().release()
    }

    fn tap_then(self, next: KeyCode) -> InputBuffer {
        self.tap().then(next)
    }

    fn press_then(self, next: KeyCode) -> InputBuffer {
        self.press().then(next)
    }

    fn hold_then(self, next: KeyCode) -> InputBuffer {
        self.hold().then(next)
    }

    fn release_then(self, next: KeyCode) -> InputBuffer {
        self.release().then(next)
    }

    fn shifted(self) -> InputBuffer {
        let code = self.code();

        let buffer = InputBuffer::new(KeyCode::KEY_LEFTSHIFT)
            .press()
            .hold()
            .then(code)
            .tap_then(KeyCode::KEY_LEFTSHIFT)
            .release();

        InputBuffer::new(code).chain(self).chain(buffer)
    }
}

#[derive(Debug, Clone)]
pub struct InputBuffer {
    current_code: KeyCode,
    buffer: Vec<InputEvent>,
}

impl InputBuffer {
    pub fn new(code: KeyCode) -> Self {
        Self {
            current_code: code,
            buffer: Vec::new(),
        }
    }

    pub fn then(mut self, code: KeyCode) -> Self {
        self.current_code = code;
        self
    }

    pub fn value(&self) -> &[InputEvent] {
        &self.buffer
    }

    pub fn chain<E: EventTarget>(mut self, other: E) -> Self {
        self.buffer.extend(other.into_value());
        self
    }

    pub fn press(code: KeyCode) -> Self {
        Self::new(code).press()
    }

    pub fn release(code: KeyCode) -> Self {
        Self::new(code).release()
    }

    pub fn tap(code: KeyCode) -> Self {
        Self::new(code).tap()
    }

    pub fn tap_hold(code: KeyCode) -> Self {
        Self::new(code).tap_hold()
    }

    pub fn combo_press(first: KeyCode, other: KeyCode) -> Self {
        Self::new(first).press_then(other).press()
    }

    pub fn combo_hold(first: KeyCode, other: KeyCode) -> Self {
        Self::new(first).hold_then(other).hold()
    }

    pub fn combo_release(first: KeyCode, other: KeyCode) -> Self {
        Self::new(first).release_then(other).release()
    }
}

impl EventTarget for InputBuffer {
    fn code(&self) -> KeyCode {
        self.current_code
    }

    fn into_value(self) -> Vec<InputEvent> {
        self.buffer
    }

    fn release(mut self) -> InputBuffer {
        self.buffer.push(release(self.current_code));
        self
    }

    fn press(mut self) -> InputBuffer {
        self.buffer.push(press(self.current_code));
        self
    }

    fn hold(mut self) -> InputBuffer {
        self.buffer.push(hold(self.current_code));
        self
    }
}

pub fn unicode() -> InputBuffer {
    InputBuffer::new(KeyCode::KEY_LEFTCTRL)
        .press_then(KeyCode::KEY_LEFTSHIFT)
        .press_then(KeyCode::KEY_U)
        .press_then(KeyCode::KEY_LEFTCTRL)
        .hold_then(KeyCode::KEY_LEFTSHIFT)
        .hold_then(KeyCode::KEY_U)
        .hold_then(KeyCode::KEY_LEFTCTRL)
        .release_then(KeyCode::KEY_LEFTSHIFT)
        .release_then(KeyCode::KEY_U)
        .release()
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

pub use anyhow::Result;
pub use evdev::KeyCode;

pub use crate::core::KeyAdapter;
