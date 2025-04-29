use ringbuffer::{ConstGenericRingBuffer as RingBuffer, RingBuffer as _};
use smallvec::SmallVec;

use crate::config::schema::KeyCode;

use super::adapter::InputResult;

#[derive(Debug, Default)]
pub struct InputBuffer {
    results: RingBuffer<InputResult, 10>,
    processed: RingBuffer<u16, 10>,
    deferred_keys: RingBuffer<KeyCode, 4>,
    pending_keys: SmallVec<[KeyCode; 4]>,
}

impl InputBuffer {
    pub fn push_result(&mut self, result: InputResult) {
        self.results.enqueue(result);
    }

    pub fn pop_result(&mut self) -> Option<InputResult> {
        self.results.dequeue()
    }

    pub fn push_key(&mut self, idx: u16) {
        self.processed.enqueue(idx);
    }

    pub fn pop_key(&mut self) -> Option<u16> {
        self.processed.dequeue()
    }

    pub fn is_pending_key(&mut self, code: &KeyCode) -> bool {
        self.pending_keys.contains(code)
    }

    pub fn has_pending_keys(&mut self) -> bool {
        !self.pending_keys.is_empty()
    }

    pub fn set_pending_key(&mut self, code: KeyCode) {
        self.pending_keys.push(code);
    }

    pub fn clear_pending_key(&mut self, code: &KeyCode) {
        self.pending_keys.retain(|value| value != code);
    }

    pub fn defer_key(&mut self, code: KeyCode) {
        self.deferred_keys.enqueue(code);
    }

    pub fn pop_deferred_key(&mut self) -> Option<KeyCode> {
        self.deferred_keys.dequeue()
    }
}
