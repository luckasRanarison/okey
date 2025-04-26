use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::{EventTarget, unicode};

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_string_macro() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_R.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_H
        .shifted()
        .then(KeyCode::KEY_I)
        .tap_then(KeyCode::KEY_COMMA)
        .tap_then(KeyCode::KEY_SPACE)
        .tap_then(KeyCode::KEY_Y)
        .tap_then(KeyCode::KEY_O)
        .tap_then(KeyCode::KEY_U)
        .tap_then(KeyCode::KEY_1)
        .shifted();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_env_macro() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    unsafe { std::env::set_var("FOO", "foo") };

    manager.process(KeyCode::KEY_E.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_F.tap_then(KeyCode::KEY_O).tap().tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_unicode_macro() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_T.tap(), &mut emitter)?;

    let expected = unicode()
        .then(KeyCode::KEY_1)
        .tap_then(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_6)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_2)
        .tap_then(KeyCode::KEY_ENTER)
        .tap()
        .chain(unicode())
        .then(KeyCode::KEY_1)
        .tap_then(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_D)
        .tap_then(KeyCode::KEY_ENTER)
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_shell_macro() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(KeyCode::KEY_Y.tap(), &mut emitter)?;

    let expected = KeyCode::KEY_F.tap_then(KeyCode::KEY_O).tap().tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
