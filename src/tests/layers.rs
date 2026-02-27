use super::utils::*;
use std::{thread, time::Duration};

const CONFIG: &str = include_str!("./config/layers.yaml");

#[test]
fn test_simple_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([
        InputSequence::Hold(KeyCode::KEY_SPACE), // first layer
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Hold(KeyCode::KEY_V), // second layer
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Release(KeyCode::KEY_V),
        InputSequence::Release(KeyCode::KEY_SPACE),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_Q),
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_P),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_combo_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::ComboPress(vec![
        KeyCode::KEY_S,
        KeyCode::KEY_D,
    ])])?;

    thread::sleep(Duration::from_millis(90));

    adapter.process_sequence([InputSequence::ComboHold(vec![
        KeyCode::KEY_S,
        KeyCode::KEY_D,
    ])])?;

    thread::sleep(Duration::from_millis(90));

    adapter.process_sequence([
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::ComboRelease(vec![KeyCode::KEY_S, KeyCode::KEY_D]),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_P),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Hold(KeyCode::KEY_A)])?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;

    adapter.process_sequence([
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Release(KeyCode::KEY_A),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_P),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_toggle_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([
        InputSequence::Tap(KeyCode::KEY_A),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_A),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_P),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_combo_toggle_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let combo = vec![KeyCode::KEY_K, KeyCode::KEY_L];

    adapter.process_sequence([
        InputSequence::ComboPress(combo.clone()),
        InputSequence::ComboRelease(combo.clone()),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::ComboPress(combo.clone()),
        InputSequence::ComboRelease(combo.clone()),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_P),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_oneshoot_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([
        InputSequence::Tap(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_P),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_X),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_X),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_reverse_release_momentary_layers() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([
        InputSequence::Hold(KeyCode::KEY_SPACE),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Hold(KeyCode::KEY_V),
        InputSequence::Release(KeyCode::KEY_SPACE),
        InputSequence::Tap(KeyCode::KEY_P),
        InputSequence::Release(KeyCode::KEY_V),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_Q),
        InputSequence::Tap(KeyCode::KEY_X),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
