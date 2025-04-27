use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/combos.yaml");

#[test]
fn test_tap_combo() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(
        InputBuffer::combo_press(KeyCode::KEY_D, KeyCode::KEY_F),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(20));

    manager.process(
        InputBuffer::combo_release(KeyCode::KEY_D, KeyCode::KEY_F),
        &mut emitter,
    )?;

    let expected = InputBuffer::tap(KeyCode::KEY_LEFTCTRL);

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_combo() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    manager.process(
        InputBuffer::combo_press(KeyCode::KEY_U, KeyCode::KEY_I),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(20));

    manager.process(
        InputBuffer::combo_release(KeyCode::KEY_U, KeyCode::KEY_I),
        &mut emitter,
    )?;

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    manager.process(
        InputBuffer::combo_press(KeyCode::KEY_U, KeyCode::KEY_I),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(90));

    manager.process(
        InputBuffer::combo_hold(KeyCode::KEY_U, KeyCode::KEY_I),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(90));

    manager.process(
        InputBuffer::combo_release(KeyCode::KEY_U, KeyCode::KEY_I),
        &mut emitter,
    )?;

    // macros should not repeat on hold
    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_expired_combo_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::press(KeyCode::KEY_D), &mut emitter)?;

    thread::sleep(Duration::from_millis(50));

    manager.process(InputBuffer::release(KeyCode::KEY_D), &mut emitter)?;

    let expected = InputBuffer::tap(KeyCode::KEY_D);

    assert_eq!(emitter.queue(), expected.value());

    emitter.clear();

    Ok(())
}

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    let expected = InputBuffer::new(KeyCode::KEY_D)
        .tap_then(KeyCode::KEY_A)
        .tap();

    manager.process(InputBuffer::press(KeyCode::KEY_D), &mut emitter)?;

    thread::sleep(Duration::from_millis(60));

    manager.process(
        InputBuffer::new(KeyCode::KEY_D)
            .release_then(KeyCode::KEY_A)
            .tap(),
        &mut emitter,
    )?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
