mod buffer;
mod combo;
mod event;
mod input;
mod manager;
mod mapping;
mod proxy;
mod tap_dance;

pub use proxy::EventProxy;

#[cfg(test)]
pub mod test_utils {
    pub use super::event::EventEmitter;
    pub use super::manager::KeyManager;
}
