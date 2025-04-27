use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/tap_dances.yaml");

#[test]
fn test_key_tap() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::tap(KeyCode::KEY_S);

    adapter.process(expected.clone())?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_key_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::press(KeyCode::KEY_S).hold())?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;
    adapter.process(InputBuffer::release(KeyCode::KEY_S))?;

    let expected = InputBuffer::tap_hold(KeyCode::KEY_LEFTSHIFT);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_tap() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_H))?;

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_I)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::press(KeyCode::KEY_H).hold())?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_Y)
        .tap();

    // macros should not repeat on hold
    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_S)
        .tap_then(KeyCode::KEY_A)
        .tap();

    adapter.process(expected.clone())?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
