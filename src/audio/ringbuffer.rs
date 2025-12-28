#![allow(dead_code)]
use std::collections::VecDeque;

pub struct AudioRingBuffer {
    buffer: VecDeque<f32>,
    max_size: usize,
    min_fill: usize,
}

impl AudioRingBuffer {
    #[must_use]
    pub fn new(max_size: usize, min_fill: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_size,
            min_fill,
        }
    }

    pub fn push(&mut self, sample: f32) {
        if self.buffer.len() >= self.max_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    pub fn pop(&mut self) -> f32 {
        if self.buffer.len() >= self.min_fill {
            self.buffer.pop_front().unwrap_or(0.0)
        } else {
            0.0
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
