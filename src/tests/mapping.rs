use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::EventTarget;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_basic_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_Q.tap_hold(), &mut emitter)?;

    let expected = KeyCode::KEY_W.tap_hold();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = KeyCode::KEY_H
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_L)
        .tap()
        .tap_then(KeyCode::KEY_O)
        .tap();

    manager.process(KeyCode::KEY_B.tap(), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(KeyCode::KEY_B.tap_hold(), &mut emitter)?;

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_event_macro() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_X.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_O.shifted().then(KeyCode::KEY_K).tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_custom_code() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_Z.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_Z.tap();

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(KeyCode::KEY_Z.press().hold(), &mut emitter)?;
    thread::sleep(Duration::from_millis(250));
    manager.post_process(&mut emitter)?;
    manager.process(KeyCode::KEY_Z.release(), &mut emitter)?;

    let expected = KeyCode::KEY_LEFTSHIFT.tap_hold();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
