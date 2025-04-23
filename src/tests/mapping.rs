use anyhow::Result;
use evdev::KeyCode;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager, hold, press, release};

#[test]
fn test_basic_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(press(KeyCode::KEY_Q), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_Q), &mut emitter)?;
    manager.process(release(KeyCode::KEY_Q), &mut emitter)?;

    let expected = vec![
        press(KeyCode::KEY_W),
        hold(KeyCode::KEY_W),
        release(KeyCode::KEY_W),
    ];

    assert_eq!(emitter.queue(), &expected);

    Ok(())
}

#[test]
fn test_macro_key() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    let expected = vec![
        press(KeyCode::KEY_H),
        release(KeyCode::KEY_H),
        press(KeyCode::KEY_E),
        release(KeyCode::KEY_E),
        press(KeyCode::KEY_L),
        release(KeyCode::KEY_L),
        press(KeyCode::KEY_L),
        release(KeyCode::KEY_L),
        press(KeyCode::KEY_O),
        release(KeyCode::KEY_O),
    ];

    manager.process(press(KeyCode::KEY_B), &mut emitter)?;
    manager.process(release(KeyCode::KEY_B), &mut emitter)?;

    assert_eq!(emitter.queue(), &expected);

    emitter.clear();

    manager.process(press(KeyCode::KEY_B), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_B), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_B), &mut emitter)?;
    manager.process(hold(KeyCode::KEY_B), &mut emitter)?;
    manager.process(release(KeyCode::KEY_B), &mut emitter)?;

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), &expected);

    Ok(())
}
