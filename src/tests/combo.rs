use super::utils::*;
use std::{thread, time::Duration};

const CONFIG: &str = include_str!("./config/combos.yaml");

#[test]
fn test_tap_combo() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let combo = vec![KeyCode::KEY_D, KeyCode::KEY_F];

    adapter.process_sequence([InputSequence::ComboPress(combo.clone())])?;

    thread::sleep(Duration::from_millis(20));

    adapter.process_sequence([InputSequence::ComboRelease(combo)])?;

    let expected = InputBuffer::new(
        [InputSequence::Tap(KeyCode::KEY_LEFTCTRL)], //
    );

    assert_eq!(proxy.queue(), expected.value());
    Ok(())
}

#[test]
fn test_macro_combo() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let combo = vec![KeyCode::KEY_U, KeyCode::KEY_I];

    adapter.process_sequence([InputSequence::ComboPress(combo.clone())])?;

    thread::sleep(Duration::from_millis(20));

    adapter.process_sequence([InputSequence::ComboRelease(combo)])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_Y),
    ]);

    assert_eq!(proxy.queue(), expected.value());
    Ok(())
}

#[test]
fn test_macro_combo_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let combo = vec![KeyCode::KEY_U, KeyCode::KEY_I];

    adapter.process_sequence([InputSequence::ComboPress(combo.clone())])?;

    thread::sleep(Duration::from_millis(90));

    adapter.process_sequence([InputSequence::ComboHold(combo.clone())])?;

    thread::sleep(Duration::from_millis(90));

    adapter.process_sequence([InputSequence::ComboRelease(combo)])?;

    // Macros should not repeat on hold
    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_Y),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_expired_combo_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Press(KeyCode::KEY_D)])?;

    thread::sleep(Duration::from_millis(50));

    adapter.process_sequence([InputSequence::Release(KeyCode::KEY_D)])?;

    let expected = InputBuffer::new([InputSequence::Tap(KeyCode::KEY_D)]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Press(KeyCode::KEY_D)])?;

    thread::sleep(Duration::from_millis(60));

    adapter.process_sequence([
        InputSequence::Release(KeyCode::KEY_D),
        InputSequence::Tap(KeyCode::KEY_A),
    ])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_D),
        InputSequence::Tap(KeyCode::KEY_A),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
