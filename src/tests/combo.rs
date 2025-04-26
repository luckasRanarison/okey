use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::EventTarget;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_tap_combo() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_D.combo_press(KeyCode::KEY_F), &mut emitter)?;
    thread::sleep(Duration::from_millis(20));
    manager.process(KeyCode::KEY_D.combo_release(KeyCode::KEY_F), &mut emitter)?;

    let expected = KeyCode::KEY_LEFTCTRL.tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_combo() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = KeyCode::KEY_H
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    manager.process(KeyCode::KEY_U.combo_press(KeyCode::KEY_I), &mut emitter)?;
    thread::sleep(Duration::from_millis(20));
    manager.process(KeyCode::KEY_U.combo_release(KeyCode::KEY_I), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(KeyCode::KEY_U.combo_press(KeyCode::KEY_I), &mut emitter)?;
    thread::sleep(Duration::from_millis(90));
    manager.process(KeyCode::KEY_U.combo_hold(KeyCode::KEY_I), &mut emitter)?;
    thread::sleep(Duration::from_millis(90));
    manager.process(KeyCode::KEY_U.combo_release(KeyCode::KEY_I), &mut emitter)?;

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_expired_combo_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_D.press(), &mut emitter)?;
    thread::sleep(Duration::from_millis(50));
    manager.process(KeyCode::KEY_D.release(), &mut emitter)?;

    let expected = KeyCode::KEY_D.tap();

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    Ok(())
}
