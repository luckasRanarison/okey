use std::{thread, time::Duration};

use super::utils::*;

const CONFIG: &str = include_str!("./config/layers.yaml");

#[test]
fn test_simple_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(
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
    )?;

    let expected = InputBuffer::new(KeyCode::KEY_Q)
        .tap_then(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_combo_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_S, KeyCode::KEY_D))?;

    thread::sleep(Duration::from_millis(90));

    adapter.process(InputBuffer::combo_hold(KeyCode::KEY_S, KeyCode::KEY_D))?;

    thread::sleep(Duration::from_millis(90));

    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::combo_release(KeyCode::KEY_S, KeyCode::KEY_D))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;

    let expected = InputBuffer::new(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_momentary_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::press(KeyCode::KEY_A).hold())?;

    thread::sleep(Duration::from_millis(250));

    adapter.post_process()?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::release(KeyCode::KEY_A))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;

    let expected = InputBuffer::new(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_tap_dance_toggle_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_A))?;

    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_E))?;

    adapter.process(InputBuffer::tap(KeyCode::KEY_A))?;

    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;

    let expected = InputBuffer::new(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_combo_toggle_layer() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_K, KeyCode::KEY_L))?;
    adapter.process(InputBuffer::combo_release(KeyCode::KEY_K, KeyCode::KEY_L))?;

    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;
    adapter.process(InputBuffer::tap(KeyCode::KEY_E))?;

    adapter.process(InputBuffer::combo_press(KeyCode::KEY_K, KeyCode::KEY_L))?;
    adapter.process(InputBuffer::combo_release(KeyCode::KEY_K, KeyCode::KEY_L))?;

    adapter.process(InputBuffer::tap(KeyCode::KEY_P))?;

    let expected = InputBuffer::new(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_X)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_P)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
