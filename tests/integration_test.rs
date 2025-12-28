#[cfg(test)]
mod tests {
    use lib::AudioRingBuffer;
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc;

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

    #[tokio::test]
    async fn test_video_channel_communication() {
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(10);

        // Spawn task to send data
        tokio::spawn(async move {
            let data = vec![1, 2, 3, 4, 5];
            tx.send(data).await.unwrap();
        });

        // Receive data
        let received = rx.recv().await.unwrap();
        assert_eq!(received, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sample_conversion() {
        // Test i16 to f32 conversion
        let max_i16: i16 = 32767;
        let min_i16: i16 = -32768;
        let zero_i16: i16 = 0;

        let max_f32 = max_i16 as f32 / 32768.0;
        let min_f32 = min_i16 as f32 / 32768.0;
        let zero_f32 = zero_i16 as f32 / 32768.0;

        assert!((max_f32 - 0.999969).abs() < 0.001);
        assert!((min_f32 - (-1.0)).abs() < 0.001);
        assert_eq!(zero_f32, 0.0);
    }

    #[test]
    fn test_sequence_wrapping() {
        let seq1: u16 = 65535;
        let seq2 = seq1.wrapping_add(1);

        assert_eq!(seq2, 0);
        assert_eq!(seq1.wrapping_add(1), 0);
    }

    #[test]
    fn test_pixel_decoding() {
        let byte: u8 = 0xAB;
        let low_nibble = byte & 0x0F;
        let high_nibble = byte >> 4;

        assert_eq!(low_nibble, 0x0B);
        assert_eq!(high_nibble, 0x0A);
    }
}
