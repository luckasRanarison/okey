use super::utils::*;

const CONFIG: &str = include_str!("./config/macros.yaml");

#[test]
fn test_macro_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_L),
        InputSequence::Tap(KeyCode::KEY_L),
        InputSequence::Tap(KeyCode::KEY_O),
    ]);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_Q)])?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_E),
        InputSequence::Tap(KeyCode::KEY_L),
        InputSequence::Tap(KeyCode::KEY_L),
        InputSequence::Tap(KeyCode::KEY_O),
    ]);

    adapter.process_sequence([InputSequence::TapHold(KeyCode::KEY_Q)])?;

    // macros should not repeat on hold
    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_event_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_X)])?;

    let expected = InputBuffer::new([
        InputSequence::Shifted(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_K),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_string_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_R)])?;

    let expected = InputBuffer::new([
        InputSequence::Shifted(KeyCode::KEY_H),
        InputSequence::Tap(KeyCode::KEY_I),
        InputSequence::Tap(KeyCode::KEY_COMMA),
        InputSequence::Tap(KeyCode::KEY_SPACE),
        InputSequence::Tap(KeyCode::KEY_Y),
        InputSequence::Tap(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_U),
        InputSequence::Shifted(KeyCode::KEY_1),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_env_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    unsafe { std::env::set_var("FOO", "foo") };

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_W)])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_F),
        InputSequence::Tap(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_O),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_unicode_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_T)])?;

    let expected = InputBuffer::new([
        InputSequence::Unicode,
        InputSequence::Tap(KeyCode::KEY_1),
        InputSequence::Tap(KeyCode::KEY_F),
        InputSequence::Tap(KeyCode::KEY_6),
        InputSequence::Tap(KeyCode::KEY_4),
        InputSequence::Tap(KeyCode::KEY_2),
        InputSequence::Tap(KeyCode::KEY_ENTER),
        InputSequence::Unicode,
        InputSequence::Tap(KeyCode::KEY_1),
        InputSequence::Tap(KeyCode::KEY_F),
        InputSequence::Tap(KeyCode::KEY_4),
        InputSequence::Tap(KeyCode::KEY_4),
        InputSequence::Tap(KeyCode::KEY_D),
        InputSequence::Tap(KeyCode::KEY_ENTER),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_shell_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_Z)])?;

    let expected = InputBuffer::new([
        InputSequence::Tap(KeyCode::KEY_F),
        InputSequence::Tap(KeyCode::KEY_O),
        InputSequence::Tap(KeyCode::KEY_O),
    ]);

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
