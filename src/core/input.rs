use std::{process::Command, str::FromStr};

use anyhow::{anyhow, Result};

use crate::config::schema::KeyCode;

use super::{
    adapter::InputResult,
    event::{IntoInputEvent, HOLD_EVENT, PRESS_EVENT, RELEASE_EVENT},
};

fn char_to_input(char: char) -> Result<InputResult> {
    let result = match char {
        char if char.is_alphabetic() | char.is_numeric() => {
            let raw_keycode = format!("KEY_{}", char.to_uppercase());
            let keycode = evdev::KeyCode::from_str(&raw_keycode).unwrap();
            Some((keycode, char.is_uppercase()))
        }

        '!' => Some((evdev::KeyCode::KEY_1, true)),
        '@' => Some((evdev::KeyCode::KEY_2, true)),
        '#' => Some((evdev::KeyCode::KEY_3, true)),
        '$' => Some((evdev::KeyCode::KEY_4, true)),
        '%' => Some((evdev::KeyCode::KEY_5, true)),
        '^' => Some((evdev::KeyCode::KEY_6, true)),
        '&' => Some((evdev::KeyCode::KEY_7, true)),
        '*' => Some((evdev::KeyCode::KEY_8, true)),
        '(' => Some((evdev::KeyCode::KEY_9, true)),
        ')' => Some((evdev::KeyCode::KEY_0, true)),

        ' ' => Some((evdev::KeyCode::KEY_SPACE, false)),
        '\t' => Some((evdev::KeyCode::KEY_TAB, false)),
        '\n' => Some((evdev::KeyCode::KEY_ENTER, false)),

        '`' => Some((evdev::KeyCode::KEY_GRAVE, false)),
        '-' => Some((evdev::KeyCode::KEY_MINUS, false)),
        '=' => Some((evdev::KeyCode::KEY_EQUAL, false)),
        '[' => Some((evdev::KeyCode::KEY_LEFTBRACE, false)),
        ']' => Some((evdev::KeyCode::KEY_RIGHTBRACE, false)),
        '\\' => Some((evdev::KeyCode::KEY_BACKSLASH, false)),
        ';' => Some((evdev::KeyCode::KEY_SEMICOLON, false)),
        '\'' => Some((evdev::KeyCode::KEY_APOSTROPHE, false)),
        ',' => Some((evdev::KeyCode::KEY_COMMA, false)),
        '.' => Some((evdev::KeyCode::KEY_DOT, false)),
        '/' => Some((evdev::KeyCode::KEY_SLASH, false)),

        '~' => Some((evdev::KeyCode::KEY_GRAVE, true)),
        '_' => Some((evdev::KeyCode::KEY_MINUS, true)),
        '+' => Some((evdev::KeyCode::KEY_EQUAL, true)),
        '{' => Some((evdev::KeyCode::KEY_LEFTBRACE, true)),
        '}' => Some((evdev::KeyCode::KEY_RIGHTBRACE, true)),
        '|' => Some((evdev::KeyCode::KEY_BACKSLASH, true)),
        ':' => Some((evdev::KeyCode::KEY_SEMICOLON, true)),
        '"' => Some((evdev::KeyCode::KEY_APOSTROPHE, true)),
        '<' => Some((evdev::KeyCode::KEY_COMMA, true)),
        '>' => Some((evdev::KeyCode::KEY_DOT, true)),
        '?' => Some((evdev::KeyCode::KEY_SLASH, true)),

        _ => None,
    };

    match result {
        Some((code, uppercase)) if uppercase => {
            let shift = KeyCode::from(evdev::KeyCode::KEY_LEFTSHIFT);
            let code = KeyCode::from(code);

            let results = vec![
                shift.to_event(PRESS_EVENT),
                shift.to_event(HOLD_EVENT),
                code.to_event(PRESS_EVENT),
                code.to_event(RELEASE_EVENT),
                shift.to_event(RELEASE_EVENT),
            ];

            Ok(InputResult::Raw(results))
        }

        Some((code, _)) => Ok(InputResult::Raw(vec![
            KeyCode::from(code).to_event(PRESS_EVENT),
            KeyCode::from(code).to_event(RELEASE_EVENT),
        ])),

        None => Err(anyhow!("Invalid character literal")),
    }
}

pub fn string_to_input(source: &str) -> Result<Vec<InputResult>> {
    source
        .chars()
        .map(char_to_input)
        .collect::<Result<Vec<_>>>()
}

pub fn unicode_to_input(source: &str, delay: u16) -> Result<Vec<InputResult>> {
    let ctrl = KeyCode::from(evdev::KeyCode::KEY_LEFTCTRL);
    let shift = KeyCode::from(evdev::KeyCode::KEY_LEFTSHIFT);
    let u = KeyCode::from(evdev::KeyCode::KEY_U);
    let enter = KeyCode::from(evdev::KeyCode::KEY_ENTER);

    let value = source
        .chars()
        .map(|c| format!("{:x}", c as u32))
        .map(|c| string_to_input(&c))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|value| {
            let ctrl_shift_u = [InputResult::Raw(vec![
                ctrl.to_event(PRESS_EVENT),
                shift.to_event(PRESS_EVENT),
                u.to_event(PRESS_EVENT),
                ctrl.to_event(HOLD_EVENT),
                shift.to_event(HOLD_EVENT),
                u.to_event(HOLD_EVENT),
                ctrl.to_event(RELEASE_EVENT),
                shift.to_event(RELEASE_EVENT),
                u.to_event(RELEASE_EVENT),
            ])];

            ctrl_shift_u
                .into_iter()
                .chain(value)
                .chain([
                    InputResult::Raw(vec![
                        enter.to_event(PRESS_EVENT),
                        enter.to_event(RELEASE_EVENT),
                    ]),
                    InputResult::Delay(delay.into()),
                ])
                .collect::<Vec<_>>()
        });

    Ok(value.collect())
}

pub fn command_to_input(command: &str, trim: Option<bool>) -> Result<Vec<InputResult>> {
    let output = Command::new("bash").arg("-c").arg(command).output()?;
    let parsed = String::from_utf8(output.stdout)?;
    let trim = trim.unwrap_or_default();
    let slice = if trim { parsed.trim() } else { &parsed };

    string_to_input(slice)
}
