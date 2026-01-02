use lib::RingBuffer;
use std::sync::{Arc, Mutex};

#[test]
fn test_audio_buffer_thread_safety() {
    let buffer = Arc::new(Mutex::new(RingBuffer::new(1000, 100)));

    let buffer_writer = buffer.clone();
    let handle = std::thread::spawn(move || {
        let mut buf = buffer_writer
            .lock()
            .expect("Failed to acquire lock on buffer");
        for i in 0..200 {
            buf.push(i as f32);
        }
    });

    handle.join().unwrap();

    let buf = buffer.lock().expect("Failed to acquire lock on buffer");
    assert_eq!(buf.len(), 200);
}
