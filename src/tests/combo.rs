use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager, press, release};

#[test]
fn test_tap_combo() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_D), &mut emitter)?;
    manager.process(press(KeyCode::KEY_F), &mut emitter)?;
    thread::sleep(Duration::from_millis(20));
    manager.process(release(KeyCode::KEY_D), &mut emitter)?;
    manager.process(release(KeyCode::KEY_F), &mut emitter)?;

    let expected = vec![press(KeyCode::KEY_LEFTCTRL), release(KeyCode::KEY_LEFTCTRL)];

    assert_eq!(emitter.queue(), &expected);

    Ok(())
}

#[test]
fn test_expired_combo_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_D), &mut emitter)?;
    thread::sleep(Duration::from_millis(50));
    manager.process(release(KeyCode::KEY_D), &mut emitter)?;

    let expected = vec![press(KeyCode::KEY_D), release(KeyCode::KEY_D)];

    assert_eq!(emitter.queue(), &expected);

    emitter.clear();

    Ok(())
}
