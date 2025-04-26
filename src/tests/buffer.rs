use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::EventTarget;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = KeyCode::KEY_D.tap_then(KeyCode::KEY_A).tap();

    manager.process(KeyCode::KEY_D.press(), &mut emitter)?;
    thread::sleep(Duration::from_millis(60));
    manager.process(
        KeyCode::KEY_D.release_then(KeyCode::KEY_A).tap(),
        &mut emitter,
    )?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = KeyCode::KEY_S.tap_then(KeyCode::KEY_A).tap();

    manager.process(expected.clone(), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
