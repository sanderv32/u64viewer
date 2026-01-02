use lib::RingBuffer;

#[test]
fn test_new_buffer_is_empty() {
    let buffer: RingBuffer<u8> = RingBuffer::new(1000, 100);
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_push_and_pop() {
    let mut buffer: RingBuffer<f32> = RingBuffer::new(1000, 10);

    buffer.push(0.5);
    buffer.push(0.3);
    buffer.push(-0.2);

    assert_eq!(buffer.len(), 3);
}

#[test]
fn test_pop_returns_silence_before_min_fill() {
    let mut buffer = RingBuffer::new(1000, 100);

    // Add samples but less than min_fill
    for _ in 0..50 {
        buffer.push(0.5);
    }

    // Should return silence because we haven't reached min_fill
    assert_eq!(buffer.pop(), 0.0);
    assert_eq!(buffer.len(), 50); // Buffer should not be consumed
}

#[test]
fn test_pop_returns_samples_after_min_fill() {
    let mut buffer = RingBuffer::new(1000, 10);

    // Fill buffer past min_fill
    for i in 0..20 {
        buffer.push(0.1 * i as f32);
    }

    // Now it should return actual samples
    assert_eq!(buffer.pop(), 0.0);
    assert_eq!(buffer.pop(), 0.1);
    assert_eq!(buffer.len(), 18);
}

#[test]
fn test_buffer_max_size_enforcement() {
    let mut buffer = RingBuffer::new(10, 5);

    // Push more samples than max_size
    for i in 0..20 {
        buffer.push(i as f32);
    }

    // Should not exceed max_size
    assert_eq!(buffer.len(), 10);
}

#[test]
fn test_buffer_drops_oldest_when_full() {
    let mut buffer = RingBuffer::new(5, 2);

    // Fill buffer
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);
    buffer.push(5);

    // Add one more - should drop the oldest (1)
    buffer.push(6);

    assert_eq!(buffer.len(), 5);
    // Pop samples - first should be 2 (1 was dropped)
    assert_eq!(buffer.pop(), 2);
}

#[test]
fn test_pop_empty_buffer_returns_zero() {
    let mut buffer: RingBuffer<u8> = RingBuffer::new(1000, 0); // min_fill = 0
    assert_eq!(buffer.pop(), 0);
}

#[test]
fn test_fifo_ordering() {
    let mut buffer = RingBuffer::new(1000, 0);

    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);

    assert_eq!(buffer.pop(), 1);
    assert_eq!(buffer.pop(), 2);
    assert_eq!(buffer.pop(), 3);
    assert_eq!(buffer.pop(), 4);
}
