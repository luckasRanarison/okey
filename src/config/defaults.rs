#[rustfmt::skip]
mod constants {
    pub fn combo_threshold() -> u16 { 50 }
    pub fn tap_dance_timeout() -> u16 { 200 }
    pub fn deferred_key_delay() -> u16 { 80 }
    pub fn unicode_input_delay() -> u16 { 50 }
}

pub use constants::*;
