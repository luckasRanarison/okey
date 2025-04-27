use std::{thread, time::Duration};

use super::utils::*;

#[test]
fn test_simple_momentary_layer() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    manager.process(
        InputBuffer::new(KeyCode::KEY_SPACE)
            .press()
            .hold_then(KeyCode::KEY_P)
            .tap_then(KeyCode::KEY_V) // second layer
            .press()
            .hold_then(KeyCode::KEY_P)
            .tap_then(KeyCode::KEY_V)
            .release_then(KeyCode::KEY_SPACE)
            .release_then(KeyCode::KEY_P)
            .tap(),
        &mut emitter,
    )?;

    let expected = InputBuffer::new(KeyCode::KEY_Q)
        .tap_then(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_combo_momentary_layer() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    manager.process(
        InputBuffer::combo_press(KeyCode::KEY_L, KeyCode::KEY_M),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(90));

    manager.process(
        InputBuffer::combo_hold(KeyCode::KEY_L, KeyCode::KEY_M),
        &mut emitter,
    )?;

    thread::sleep(Duration::from_millis(90));

    manager.process(InputBuffer::tap(KeyCode::KEY_P), &mut emitter)?;

    manager.process(
        InputBuffer::combo_release(KeyCode::KEY_L, KeyCode::KEY_M),
        &mut emitter,
    )?;

    manager.process(InputBuffer::tap(KeyCode::KEY_P), &mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
