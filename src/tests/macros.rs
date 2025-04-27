use super::utils::*;

const CONFIG: &str = include_str!("./config/macros.yaml");

#[test]
fn test_event_macro() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap(KeyCode::KEY_X), &mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_O)
        .shifted()
        .then(KeyCode::KEY_K)
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_string_macro() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap(KeyCode::KEY_R), &mut emitter)?;

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

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_env_macro() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    unsafe { std::env::set_var("FOO", "foo") };

    manager.process(InputBuffer::tap(KeyCode::KEY_E), &mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_O)
        .tap()
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_unicode_macro() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap(KeyCode::KEY_T), &mut emitter)?;

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

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}

#[test]
fn test_shell_macro() -> Result<()> {
    let mut emitter = BufferedEventEmitter::default();
    let mut manager = KeyManager::with_config(CONFIG);

    manager.process(InputBuffer::tap(KeyCode::KEY_Y), &mut emitter)?;

    let expected = InputBuffer::new(KeyCode::KEY_F)
        .tap_then(KeyCode::KEY_O)
        .tap()
        .tap();

    assert_eq!(emitter.queue(), expected.value());

    Ok(())
}
