#[rustfmt::skip]
mod constants {
    pub fn event_poll_timeout() -> u16 { 1 }
    pub fn combo_threshold() -> u16 { 10 }
    pub fn tap_dance_timeout() -> u16 { 200 }
    pub fn deferred_key_delay() -> u16 { 0 }
    pub fn unicode_input_delay() -> u16 { 50 }
    pub fn maximum_lookup_depth() -> u8 { 10 }
}

pub use constants::*;
