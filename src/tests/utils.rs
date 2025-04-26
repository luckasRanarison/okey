use anyhow::Result;
use evdev::{EventType, InputEvent, KeyCode};

use crate::{
    config::schema::Config,
    core::test_utils::{EventEmitter, KeyManager},
};

const CONFIG: &str = include_str!("./config/okey.yaml");

pub trait EventProcessor {
    fn process_input<E: EventEmitter>(&mut self, event: InputEvent, emitter: &mut E) -> Result<()>;

    fn process<E: EventEmitter>(&mut self, buffer: InputBuffer, emitter: &mut E) -> Result<()> {
        for event in buffer.value() {
            self.process_input(*event, emitter)?;
        }

        Ok(())
    }
}

impl EventProcessor for KeyManager {
    fn process_input<E: EventEmitter>(&mut self, event: InputEvent, emitter: &mut E) -> Result<()> {
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
        self.queue.extend(events);
        Ok(())
    }
}

pub fn get_test_manager() -> KeyManager {
    let mut config: Config = serde_yaml::from_str(CONFIG).unwrap();
    let keyboard = config.keyboards.remove(0);
    let defaults = config.defaults.clone();

    KeyManager::new(keyboard, defaults)
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

        let buffer = KeyCode::KEY_LEFTSHIFT
            .press()
            .hold()
            .then(code)
            .tap_then(KeyCode::KEY_LEFTSHIFT)
            .release();

        InputBuffer::new(self.into_value(), Some(code)).chain(buffer)
    }

    fn combo_press<E: EventTarget>(self, other: E) -> InputBuffer {
        self.press_then(other.code()).press()
    }

    fn combo_hold<E: EventTarget>(self, other: E) -> InputBuffer {
        self.hold_then(other.code()).hold()
    }

    fn combo_release<E: EventTarget>(self, other: E) -> InputBuffer {
        self.release_then(other.code()).release()
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputBuffer {
    current_code: Option<KeyCode>,
    buffer: Vec<InputEvent>,
}

impl InputBuffer {
    fn new(buffer: Vec<InputEvent>, current_code: Option<KeyCode>) -> Self {
        Self {
            current_code,
            buffer,
        }
    }

    pub fn then(mut self, code: KeyCode) -> Self {
        self.current_code = Some(code);
        self
    }

    pub fn value(&self) -> &[InputEvent] {
        &self.buffer
    }

    pub fn chain(mut self, other: Self) -> Self {
        self.buffer.extend(other.buffer);
        self
    }
}

impl EventTarget for InputBuffer {
    fn code(&self) -> KeyCode {
        self.current_code.unwrap()
    }

    fn into_value(self) -> Vec<InputEvent> {
        self.buffer
    }

    fn release(mut self) -> InputBuffer {
        if let Some(code) = self.current_code {
            self.buffer.push(release(code));
        }

        self
    }

    fn press(mut self) -> InputBuffer {
        if let Some(code) = self.current_code {
            self.buffer.push(press(code));
        }

        self
    }

    fn hold(mut self) -> InputBuffer {
        if let Some(code) = self.current_code {
            self.buffer.push(hold(code));
        }

        self
    }
}

impl EventTarget for KeyCode {
    fn code(&self) -> KeyCode {
        *self
    }

    fn into_value(self) -> Vec<InputEvent> {
        vec![]
    }

    fn press(self) -> InputBuffer {
        InputBuffer::new(vec![press(self)], Some(self))
    }

    fn hold(self) -> InputBuffer {
        InputBuffer::new(vec![hold(self)], Some(self))
    }

    fn release(self) -> InputBuffer {
        InputBuffer::new(vec![release(self)], Some(self))
    }
}

pub fn unicode() -> InputBuffer {
    KeyCode::KEY_LEFTCTRL
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
