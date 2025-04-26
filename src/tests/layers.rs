use anyhow::Result;
use evdev::KeyCode;

use crate::tests::utils::EventTarget;

use super::utils::{EventProcessor, FakeEventEmitter, get_test_manager};

#[test]
fn test_simple_momentary_layer() -> Result<()> {
    let mut emitter = FakeEventEmitter::default();
    let mut manager = get_test_manager();

    manager.process(
        KeyCode::KEY_SPACE
            .press()
            .hold_then(KeyCode::KEY_P)
            .tap_then(KeyCode::KEY_V) // second layer
            .press()
            .hold_then(KeyCode::KEY_P)
            .tap(),
        &mut emitter,
    )?;

    let expected = KeyCode::KEY_Q.tap_then(KeyCode::KEY_X).tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
