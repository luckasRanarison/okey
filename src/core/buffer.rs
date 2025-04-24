use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

use super::manager::InputResult;

const BUFFER_SIZE: usize = 10;

#[derive(Debug, Default)]
pub struct InputBuffer {
    results: ConstGenericRingBuffer<InputResult, BUFFER_SIZE>,
    processed: ConstGenericRingBuffer<u16, BUFFER_SIZE>,
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
}
