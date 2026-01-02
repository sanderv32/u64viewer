#![allow(dead_code)]
use std::collections::VecDeque;

pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    max_size: usize,
    min_fill: usize,
}

impl<T> RingBuffer<T>
where
    T: Default
{
    #[must_use]
    pub fn new(max_size: usize, min_fill: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_size,
            min_fill,
        }
    }

    pub fn push(&mut self, sample: T) {
        if self.buffer.len() >= self.max_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    pub fn pop(&mut self) -> T {
        if self.buffer.len() >= self.min_fill {
            self.buffer.pop_front().unwrap_or_default()
        } else {
            T::default()
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }
}
