use std::{thread, time::Duration};

use super::utils::*;

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

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

#[test]
fn test_tap_dance_key() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::default();

    let expected = InputBuffer::new(KeyCode::KEY_S)
        .tap_then(KeyCode::KEY_A)
        .tap();

    manager.process(expected.clone(), &mut emitter)?;

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
