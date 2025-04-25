use std::collections::HashSet;

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

use crate::config::schema::KeyCode;

use super::manager::InputResult;

const BUFFER_SIZE: usize = 10;
const DEFER_BUFFER_SIZE: usize = 3;

#[derive(Debug, Default)]
pub struct InputBuffer {
    results: ConstGenericRingBuffer<InputResult, BUFFER_SIZE>,
    processed: ConstGenericRingBuffer<u16, BUFFER_SIZE>,
    defered_keys: ConstGenericRingBuffer<KeyCode, DEFER_BUFFER_SIZE>,
    pending_keys: HashSet<KeyCode>,
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
        self.pending_keys.insert(code);
    }

    pub fn clear_pending_key(&mut self, code: &KeyCode) {
        self.pending_keys.remove(code);
    }

    pub fn defer_key(&mut self, code: KeyCode) {
        self.defered_keys.enqueue(code);
    }

    pub fn pop_defered_key(&mut self) -> Option<KeyCode> {
        self.defered_keys.dequeue()
    }
}
