use std::io;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tracing::debug;
use zerocopy::{FromBytes, Immutable, KnownLayout};

use crate::AudioBuffer;
use crate::CANCEL_TOKEN;

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, Immutable, KnownLayout)]
pub struct VideoStream {
    seq: u16,        // Sequence number
    frame: u16,      // Frame number
    line: u16,       // Line number
    width: u16,      // Pixels per line (always 384)
    lpp: u8,         // Lines per packet (always 4)
    bits: u8,        // Bit per pixel (always 4)
    encoding: u16,   // Encoding type (0=no encoding, 1=RLE encoding)
    data: [u8; 768], // Frame data
}

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, Immutable, KnownLayout)]
pub struct AudioStream {
    seq: u16,              // Sequence number
    data: [[i16; 2]; 192], // Left channel, Right channel
}

pub async fn handle_video(socket: UdpSocket, sender: mpsc::Sender<Vec<u8>>) -> io::Result<()> {
    debug!("Starting video handler");
    let mut buf = vec![0u8; 780];
    let mut first_run = true;

    while !CANCEL_TOKEN.is_cancelled() {
        let mut frame_data = Vec::new();

        loop {
            let (len, src) = socket.recv_from(&mut buf).await?;
            debug!("Video: {} bytes from {}", len, src);
            let video_stream = match VideoStream::read_from_bytes(&buf) {
                Ok(v) => v,
                Err(e) => {
                    debug!("Failed to parse video stream: {e:?}");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid video packet: {e:?}")
                    ));
                }
            };
            if video_stream.line & 0x8000 != 0 {
                first_run = false;
            }

            if !first_run {
                frame_data.extend_from_slice(&video_stream.data);

                if video_stream.line & 0x8000 != 0 {
                    if (sender.send(frame_data).await).is_err() {
                        debug!("Receiver dropped");
                    }
                    break;
                }
            } else if video_stream.line & 0x8000 != 0 {
                break;
            }
        }
    }
    Ok(())
}

pub async fn handle_audio(socket: UdpSocket, audio_buffer: AudioBuffer) -> io::Result<()> {
    debug!("Starting audio handler");
    let mut previous_seq: Option<u16> = None;
    let mut buf = vec![0u8; 770];
    while !CANCEL_TOKEN.is_cancelled() {
        let (len, src) = socket.recv_from(&mut buf).await?;
        debug!("Audio: {} bytes from {}", len, src);
        // Process audio packet
        let audio_stream = match AudioStream::read_from_bytes(&buf) {
            Ok(a) => a,
            Err(e) => {
                debug!("Failed to parse audio stream: {e:?}");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid audio packet: {e:?}")
                ));
            }
        };

        // Check for dropped packets
        if let Some(prev) = previous_seq
            && audio_stream.seq != prev.wrapping_add(1)
        {
            debug!(
                "Dropped audio packet! Expected {}, got {}",
                prev.wrapping_add(1),
                audio_stream.seq,
            );

            let mut buffer = audio_buffer
                .lock()
                .expect("Unable to acquire lock on audio_buffer");
            for _ in 0..384 {
                buffer.push(0.);
            }
        }
        previous_seq = Some(audio_stream.seq);

        let mut buffer = audio_buffer
            .lock()
            .expect("Unable to acquire lock on audio_buffer");

        for sample_pair in &audio_stream.data {
            let left = f32::from(sample_pair[0]) / 32768.;
            let right = f32::from(sample_pair[1]) / 32768.;
            buffer.push(left);
            buffer.push(right);
        }
    }
    Ok(())
}
