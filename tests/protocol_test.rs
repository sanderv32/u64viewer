#[cfg(test)]
mod tests {
    use zerocopy::FromBytes;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, FromBytes)]
    struct VideoStream {
        seq: u16,
        frame: u16,
        line: u16,
        width: u16,
        lpp: u8,
        bits: u8,
        encoding: u16,
        data: [u8; 768],
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, FromBytes)]
    struct AudioStream {
        seq: u16,
        data: [[i16; 2]; 192],
    }

    #[test]
    fn test_video_stream_size() {
        assert_eq!(std::mem::size_of::<VideoStream>(), 780);
    }

    #[test]
    fn test_audio_stream_size() {
        assert_eq!(std::mem::size_of::<AudioStream>(), 770);
    }

    #[test]
    fn test_video_stream_parsing() {
        let mut buf = vec![0u8; 780];

        // Set up header
        buf[0..2].copy_from_slice(&1u16.to_le_bytes()); // seq
        buf[2..4].copy_from_slice(&5u16.to_le_bytes()); // frame
        buf[4..6].copy_from_slice(&0x8000u16.to_le_bytes()); // line with end marker
        buf[6..8].copy_from_slice(&384u16.to_le_bytes()); // width

        let stream = VideoStream::read_from_bytes(&buf).unwrap();
        assert_eq!(stream.seq, 1);
        assert_eq!(stream.frame, 5);
        assert_eq!(stream.line & 0x8000, 0x8000); // Check end marker
        assert_eq!(stream.width, 384);
    }

    #[test]
    fn test_audio_stream_parsing() {
        let mut buf = vec![0u8; 770];

        // Set sequence number
        buf[0..2].copy_from_slice(&42u16.to_le_bytes());

        // Set first stereo sample
        let left: i16 = 1000;
        let right: i16 = -1000;
        buf[2..4].copy_from_slice(&left.to_le_bytes());
        buf[4..6].copy_from_slice(&right.to_le_bytes());

        let stream = AudioStream::read_from_bytes(&buf).unwrap();
        assert_eq!(stream.seq, 42);
        assert_eq!(stream.data[0][0], 1000);
        assert_eq!(stream.data[0][1], -1000);
    }

    #[test]
    fn test_video_end_marker_detection() {
        let line_normal = 0x0010u16;
        let line_end = 0x8010u16;

        assert_eq!(line_normal & 0x8000, 0);
        assert_eq!(line_end & 0x8000, 0x8000);
    }
}
