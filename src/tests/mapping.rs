use super::utils::*;

const CONFIG: &str = include_str!("./config/mappings.yaml");

#[test]
fn test_basic_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::TapHold(KeyCode::KEY_Q)])?;

    let expected = InputBuffer::new(
        [InputSequence::TapHold(KeyCode::KEY_W)], //
    );

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_custom_code() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process_sequence([InputSequence::Tap(KeyCode::KEY_Z)])?;

    let expected = InputBuffer::new(
        [InputSequence::Tap(KeyCode::KEY_A)], //
    );

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
