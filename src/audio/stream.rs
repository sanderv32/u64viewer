use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use tracing::{debug, error};

use crate::RingBuffer;

pub type AudioBuffer = Arc<Mutex<RingBuffer<f32>>>;

/// # Panics
/// Panics if no output device is found or if unable to acquire a `audio_buffer` lock
/// # Errors
/// Return an error if stream cannot be played
pub fn init_audio(audio_buffer: &AudioBuffer) -> Result<Stream, String> {
    debug!("Initializing audio");
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");

    let audio_callback = {
        let buffer = audio_buffer.clone();
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut buf = buffer.lock().expect("Unable to acquire lock on audio_buffer");
            for sample in data.iter_mut() {
                *sample = buf.pop();
            }
        }
    };

    let desired_config = StreamConfig {
        channels: 2,
        sample_rate: 48_000,
        buffer_size: cpal::BufferSize::Default,
    };

    let stream = match device.build_output_stream(
        &desired_config,
        audio_callback.clone(),
        |err| error!("Audio stream error: {err}"),
        None,
    ) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to create stream with 48000 Hz, trying 44100 Hz: {e}");
            let fallback_config = StreamConfig {
                channels: 2,
                sample_rate: 44_100,
                buffer_size: cpal::BufferSize::Default,
            };
            device
                .build_output_stream(
                    &fallback_config,
                    audio_callback,
                    |err| error!("Audio stream error: {err}"),
                    None,
                )
                .expect("Failed to create audio stream")
        }
    };
    stream.play().map_err(|_| "Unable to start audio stream")?;
    Ok(stream)
}
