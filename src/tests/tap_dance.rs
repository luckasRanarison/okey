use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::EventTarget;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_key_tap() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = KeyCode::KEY_S.tap();

    manager.process(expected.clone(), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_key_hold() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(
        KeyCode::KEY_S.press_then(KeyCode::KEY_S).hold(),
        &mut emitter,
    )?;
    thread::sleep(Duration::from_millis(250));
    manager.post_process(&mut emitter)?;
    manager.process(KeyCode::KEY_S.release(), &mut emitter)?;

    let expected = KeyCode::KEY_LEFTSHIFT.tap_hold();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_tap() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_H.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_H.tap_then(KeyCode::KEY_I).tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_H.press().hold(), &mut emitter)?;
    thread::sleep(Duration::from_millis(250));
    manager.post_process(&mut emitter)?;

    let expected = KeyCode::KEY_H
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
