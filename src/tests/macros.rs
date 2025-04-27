use super::utils::*;

const CONFIG: &str = include_str!("./config/macros.yaml");

#[test]
fn test_macro_key() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_L)
        .tap()
        .tap_then(KeyCode::KEY_O)
        .tap();

    adapter.process(InputBuffer::tap(KeyCode::KEY_Q))?;

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_macro_hold() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .tap_then(KeyCode::KEY_E)
        .tap_then(KeyCode::KEY_L)
        .tap()
        .tap_then(KeyCode::KEY_O)
        .tap();

    adapter.process(InputBuffer::tap_hold(KeyCode::KEY_Q))?;

    // macros should not repeat on hold
    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_event_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_X))?;

    let expected = InputBuffer::new(KeyCode::KEY_O)
        .shifted()
        .then(KeyCode::KEY_K)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_string_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_R))?;

    let expected = InputBuffer::new(KeyCode::KEY_H)
        .shifted()
        .then(KeyCode::KEY_I)
        .tap_then(KeyCode::KEY_COMMA)
        .tap_then(KeyCode::KEY_SPACE)
        .tap_then(KeyCode::KEY_Y)
        .tap_then(KeyCode::KEY_O)
        .tap_then(KeyCode::KEY_U)
        .tap_then(KeyCode::KEY_1)
        .shifted();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_env_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    unsafe { std::env::set_var("FOO", "foo") };

    adapter.process(InputBuffer::tap(KeyCode::KEY_W))?;

    let expected = InputBuffer::new(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_O)
        .tap()
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_unicode_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_T))?;

    let expected = unicode()
        .then(KeyCode::KEY_1)
        .tap_then(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_6)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_2)
        .tap_then(KeyCode::KEY_ENTER)
        .tap()
        .chain(unicode())
        .then(KeyCode::KEY_1)
        .tap_then(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_4)
        .tap_then(KeyCode::KEY_D)
        .tap_then(KeyCode::KEY_ENTER)
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}

#[test]
fn test_shell_macro() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::tap(KeyCode::KEY_Z))?;

    let expected = InputBuffer::new(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_O)
        .tap()
        .tap();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
