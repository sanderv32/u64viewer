mod stream;
use crate::RingBuffer;

pub use stream::init_audio;

use std::sync::{Arc, Mutex};
pub type AudioBuffer = Arc<Mutex<RingBuffer<f32>>>;
