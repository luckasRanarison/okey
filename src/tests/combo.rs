use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/combos.yaml");

#[test]
fn test_tap_combo() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_D, KeyCode::KEY_F))?;

    thread::sleep(Duration::from_millis(20));

    adapter.process(InputBuffer::combo_release(KeyCode::KEY_D, KeyCode::KEY_F))?;

    let expected = InputBuffer::tap(KeyCode::KEY_LEFTCTRL);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_combo() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_U, KeyCode::KEY_I))?;

    thread::sleep(Duration::from_millis(20));

    adapter.process(InputBuffer::combo_release(KeyCode::KEY_U, KeyCode::KEY_I))?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_combo_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_U, KeyCode::KEY_I))?;

    thread::sleep(Duration::from_millis(90));

    adapter.process(InputBuffer::combo_hold(KeyCode::KEY_U, KeyCode::KEY_I))?;

    thread::sleep(Duration::from_millis(90));

    adapter.process(InputBuffer::combo_release(KeyCode::KEY_U, KeyCode::KEY_I))?;

    // macros should not repeat on hold
    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_expired_combo_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::press(KeyCode::KEY_D))?;

    thread::sleep(Duration::from_millis(50));

    adapter.process(InputBuffer::release(KeyCode::KEY_D))?;

    let expected = InputBuffer::tap(KeyCode::KEY_D);

    assert_eq!(proxy.queue(), expected.value());

    proxy.clear();

    Ok(())
}

#[test]
fn test_derred_combo_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_D)
        .tap_then(KeyCode::KEY_A)
        .tap();

    adapter.process(InputBuffer::press(KeyCode::KEY_D))?;

    thread::sleep(Duration::from_millis(60));

    adapter.process(
        InputBuffer::new(KeyCode::KEY_D)
            .release_then(KeyCode::KEY_A)
            .tap(),
    )?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
