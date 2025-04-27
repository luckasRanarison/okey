use std::{process::Command, str::FromStr};

use anyhow::{Result, anyhow};
use evdev::KeyCode;

use super::adapter::InputResult;

fn char_to_input(char: char) -> Result<Vec<InputResult>> {
    let result = match char {
        char if char.is_alphabetic() | char.is_numeric() => {
            let raw_keycode = format!("KEY_{}", char.to_uppercase());
            let keycode = KeyCode::from_str(&raw_keycode).unwrap();
            Some((keycode, char.is_uppercase()))
        }

        '!' => Some((KeyCode::KEY_1, true)),
        '@' => Some((KeyCode::KEY_2, true)),
        '#' => Some((KeyCode::KEY_3, true)),
        '$' => Some((KeyCode::KEY_4, true)),
        '%' => Some((KeyCode::KEY_5, true)),
        '^' => Some((KeyCode::KEY_6, true)),
        '&' => Some((KeyCode::KEY_7, true)),
        '*' => Some((KeyCode::KEY_8, true)),
        '(' => Some((KeyCode::KEY_9, true)),
        ')' => Some((KeyCode::KEY_0, true)),

        ' ' => Some((KeyCode::KEY_SPACE, false)),
        '\t' => Some((KeyCode::KEY_TAB, false)),
        '\n' => Some((KeyCode::KEY_ENTER, false)),

        '`' => Some((KeyCode::KEY_GRAVE, false)),
        '-' => Some((KeyCode::KEY_MINUS, false)),
        '=' => Some((KeyCode::KEY_EQUAL, false)),
        '[' => Some((KeyCode::KEY_LEFTBRACE, false)),
        ']' => Some((KeyCode::KEY_RIGHTBRACE, false)),
        '\\' => Some((KeyCode::KEY_BACKSLASH, false)),
        ';' => Some((KeyCode::KEY_SEMICOLON, false)),
        '\'' => Some((KeyCode::KEY_APOSTROPHE, false)),
        ',' => Some((KeyCode::KEY_COMMA, false)),
        '.' => Some((KeyCode::KEY_DOT, false)),
        '/' => Some((KeyCode::KEY_SLASH, false)),

        '~' => Some((KeyCode::KEY_GRAVE, true)),
        '_' => Some((KeyCode::KEY_MINUS, true)),
        '+' => Some((KeyCode::KEY_EQUAL, true)),
        '{' => Some((KeyCode::KEY_LEFTBRACE, true)),
        '}' => Some((KeyCode::KEY_RIGHTBRACE, true)),
        '|' => Some((KeyCode::KEY_BACKSLASH, true)),
        ':' => Some((KeyCode::KEY_SEMICOLON, true)),
        '"' => Some((KeyCode::KEY_APOSTROPHE, true)),
        '<' => Some((KeyCode::KEY_COMMA, true)),
        '>' => Some((KeyCode::KEY_DOT, true)),
        '?' => Some((KeyCode::KEY_SLASH, true)),

        _ => None,
    };

    match result {
        Some((code, uppercase)) if uppercase => {
            let shift = KeyCode::KEY_LEFTSHIFT.into();
            let code = code.into();

            let action = vec![
                InputResult::Press(shift),
                InputResult::Hold(shift),
                InputResult::Press(code),
                InputResult::Release(code),
                InputResult::Release(shift),
            ];

            Ok(action)
        }
        Some((code, _)) => Ok(vec![
            InputResult::Press(code.into()),
            InputResult::Release(code.into()),
        ]),
        None => Err(anyhow!("Invalid character literal")),
    }
}

pub fn string_to_input(source: &str) -> Result<Vec<InputResult>> {
    source
        .chars()
        .map(char_to_input)
        .collect::<Result<Vec<_>>>()
        .map(|res| res.into_iter().flatten().collect())
}

pub fn unicode_to_input(source: &str, delay: u16) -> Result<Vec<InputResult>> {
    let ctrl = KeyCode::KEY_LEFTCTRL.into();
    let shift = KeyCode::KEY_LEFTSHIFT.into();
    let u = KeyCode::KEY_U.into();
    let enter = KeyCode::KEY_ENTER.into();

    let value = source
        .chars()
        .map(|c| format!("{:x}", c as u32))
        .map(|c| string_to_input(&c))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|value| {
            vec![
                InputResult::Press(ctrl),
                InputResult::Press(shift),
                InputResult::Press(u),
                InputResult::Hold(ctrl),
                InputResult::Hold(shift),
                InputResult::Hold(u),
                InputResult::Release(ctrl),
                InputResult::Release(shift),
                InputResult::Release(u),
            ]
            .into_iter()
            .chain(value)
            .chain(vec![
                InputResult::Press(enter),
                InputResult::Release(enter),
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
