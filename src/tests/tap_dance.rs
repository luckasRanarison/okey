use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/tap_dances.yaml");

#[test]
fn test_key_tap() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(
        [InputSequence::Tap(KeyCode::KEY_S)], //
    );

    adapter.process_buffer(&expected)?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_key_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Hold(KeyCode::KEY_S)])?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;
    adapter.process_sequence([InputSequence::Release(KeyCode::KEY_S)])?;

    let expected = InputBuffer::new(
        [InputSequence::TapHold(KeyCode::KEY_LEFTSHIFT)], //
    );

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_tap() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_H)])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_I),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Hold(KeyCode::KEY_H)])?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_Y),
    ]);

    // macros should not repeat on hold
    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_S),
        InputSequence::Tap(KeyCode::KEY_A),
    ]);

    adapter.process_buffer(&expected)?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
