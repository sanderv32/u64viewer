#[cfg(test)]
mod tests {
    use lib::AudioRingBuffer;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_audio_buffer_thread_safety() {
        let buffer = Arc::new(Mutex::new(AudioRingBuffer::new(1000, 100)));

        let buffer_clone = buffer.clone();
        let handle = std::thread::spawn(move || {
            let mut buf = buffer_clone.lock().unwrap();
            for i in 0..200 {
                buf.push(i as f32);
            }
        });

        handle.join().unwrap();

        let buf = buffer.lock().unwrap();
        assert_eq!(buf.len(), 200);
    }
}
