use std::{thread, time::Duration};

use anyhow::Result;
use evdev::KeyCode;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager, hold, press, release};

#[test]
fn test_key_tap() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_S), &mut emitter)?;
    manager.process(release(KeyCode::KEY_S), &mut emitter)?;

    let expected = vec![press(KeyCode::KEY_S), release(KeyCode::KEY_S)];

    assert_eq!(emitter.queue(), &expected);

    Ok(())
}

#[test]
fn test_key_hold() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_S), &mut emitter)?;
    manager.process_event(hold(KeyCode::KEY_S), &mut emitter)?;
    thread::sleep(Duration::from_millis(250));
    manager.post_process(&mut emitter)?;
    manager.process(release(KeyCode::KEY_S), &mut emitter)?;

    let expected = vec![
        press(KeyCode::KEY_LEFTSHIFT),
        hold(KeyCode::KEY_LEFTSHIFT),
        release(KeyCode::KEY_LEFTSHIFT),
    ];

    assert_eq!(emitter.queue(), &expected);

    Ok(())
}

#[test]
fn test_macro_tap() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_H), &mut emitter)?;
    manager.process(release(KeyCode::KEY_H), &mut emitter)?;

    let expected = vec![
        press(KeyCode::KEY_H),
        release(KeyCode::KEY_H),
        press(KeyCode::KEY_I),
        release(KeyCode::KEY_I),
    ];

    assert_eq!(emitter.queue(), &expected);

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_H), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_H), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_H), &mut emitter)?;
    manager.process_event(hold(KeyCode::KEY_H), &mut emitter)?;
    thread::sleep(Duration::from_millis(250));
    manager.post_process(&mut emitter)?;

    let expected = vec![
        press(KeyCode::KEY_H),
        release(KeyCode::KEY_H),
        press(KeyCode::KEY_E),
        release(KeyCode::KEY_E),
        press(KeyCode::KEY_Y),
        release(KeyCode::KEY_Y),
    ];

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), &expected);

    Ok(())
}
