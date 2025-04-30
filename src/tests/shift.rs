use crate::config::schema::{Config, KeyAction};

use super::utils::*;

const CONFIG: &str = include_str!("./config/shift.yaml");

#[test]
fn test_shifted_keycodes() {
    let mut config: Config = serde_yaml::from_str(CONFIG).unwrap();
    let keyboard = config.keyboards.remove(0);

    for (key, action) in keyboard.keys {
        if let KeyAction::KeyCode(code) = action {
            assert!(code.is_shifted())
        } else {
            panic!("Non shifted keycode mapped to {key:?}")
        }
    }
}

#[test]
fn test_shifted_key_event() -> Result<()> {
    let mut proxy = EventProxyMock::default();
    let mut adapter = KeyAdapter::with_config(CONFIG, &mut proxy);

    adapter.process(InputBuffer::press(KeyCode::KEY_Q).hold().release())?;

    let expected = InputBuffer::new(KeyCode::KEY_LEFTSHIFT)
        .press()
        .hold_then(KeyCode::KEY_1)
        .press()
        .hold_then(KeyCode::KEY_LEFTSHIFT)
        .release_then(KeyCode::KEY_1)
        .release();

    assert_eq!(proxy.queue(), expected.value());

    Ok(())
}
