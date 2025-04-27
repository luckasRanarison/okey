use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/mappings.yaml");

#[test]
fn test_basic_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap_hold(KeyCode::KEY_Q), &mut emitter)?;

    let expected = InputBuffer::tap_hold(KeyCode::KEY_W);

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_L)
        .tap()
        .tap_then(KeyCode::KEY_O)
        .tap();

    manager.process(InputBuffer::tap(KeyCode::KEY_B), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(InputBuffer::tap_hold(KeyCode::KEY_B), &mut emitter)?;

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_custom_code() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap(KeyCode::KEY_Z), &mut emitter)?;

    let expected = InputBuffer::tap(KeyCode::KEY_Z);

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(InputBuffer::press(KeyCode::KEY_Z).hold(), &mut emitter)?;

    thread::sleep(Duration::from_millis(250));

    manager.post_process(&mut emitter)?;
    manager.process(InputBuffer::release(KeyCode::KEY_Z), &mut emitter)?;

    let expected = InputBuffer::tap_hold(KeyCode::KEY_LEFTSHIFT);

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
