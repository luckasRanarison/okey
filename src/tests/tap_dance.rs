use std::{thread, time::Duration};

use super::utils::*;

#[test]
fn test_key_tap() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    let expected = InputBuffer::tap(KeyCode::KEY_S);

    manager.process(expected.clone(), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_key_hold() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    manager.process(InputBuffer::press(KeyCode::KEY_S).hold(), &mut emitter)?;

    thread::sleep(Duration::from_millis(250));

    manager.post_process(&mut emitter)?;
    manager.process(InputBuffer::release(KeyCode::KEY_S), &mut emitter)?;

    let expected = InputBuffer::tap_hold(KeyCode::KEY_LEFTSHIFT);

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_tap() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    manager.process(InputBuffer::tap(KeyCode::KEY_H), &mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_I)
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    manager.process(InputBuffer::press(KeyCode::KEY_H).hold(), &mut emitter)?;

    thread::sleep(Duration::from_millis(250));

    manager.post_process(&mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
