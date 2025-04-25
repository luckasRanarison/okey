use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager, press, release};

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = vec![
        press(KeyCode::KEY_D),
        release(KeyCode::KEY_D),
        press(KeyCode::KEY_A),
        release(KeyCode::KEY_A),
    ];

    manager.process(press(KeyCode::KEY_D), &mut emitter)?;
    thread::sleep(Duration::from_millis(60));
    manager.process(release(KeyCode::KEY_D), &mut emitter)?;
    manager.process(press(KeyCode::KEY_A), &mut emitter)?;
    manager.process(release(KeyCode::KEY_A), &mut emitter)?;

    assert_eq!(emitter.queue(), expected);

    Ok(())
}

#[test]
fn test_tap_dance_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = vec![
        press(KeyCode::KEY_S),
        release(KeyCode::KEY_S),
        press(KeyCode::KEY_A),
        release(KeyCode::KEY_A),
    ];

    manager.process(press(KeyCode::KEY_S), &mut emitter)?;
    manager.process(release(KeyCode::KEY_S), &mut emitter)?;
    manager.process(press(KeyCode::KEY_A), &mut emitter)?;
    manager.process(release(KeyCode::KEY_A), &mut emitter)?;

    assert_eq!(emitter.queue(), expected);

    Ok(())
}
