use clap::Parser;
use std::{
    sync::{Arc, LazyLock, Mutex},
    thread,
};
use tokio::sync::mpsc::{self};
use tokio_util::sync::CancellationToken;

mod args;
mod audio;
mod constants;
mod network;
mod video;
use crate::{
    args::Args,
    audio::{AudioBuffer, AudioRingBuffer},
    network::NetworkConfig,
    video::Window,
};

static CANCEL_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let (width, height) = args.dimensions;
    if !args.palette.is_empty() && args.palette.len() != 16 {
        eprintln!(
            "Error: Pallete must have exactly 16 colors, got {}",
            args.palette.len()
        );
        std::process::exit(1);
    }
    let palette: Option<Vec<u32>> = if args.palette.is_empty() {
        None
    } else {
        Some(args.palette)
    };

    let (audio_buffer, _stream) = if args.mute {
        (None, None)
    } else {
        let buffer = Arc::new(Mutex::new(AudioRingBuffer::new(48_000, 12_000)));
        let stream = audio::init_audio(buffer.clone());
        (Some(buffer), Some(stream))
    };

    // Create channel for video buffer updates
    let (video_tx, mut video_rx) = mpsc::channel::<Vec<u8>>(20);

    // Spawn concurrent tasks
    let network_config = NetworkConfig {
        video_maddr: args.video_maddr,
        audio_maddr: args.audio_maddr,
        video_port: args.video_port,
        audio_port: args.audio_port,
    };
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            network::network_tasks(network_config, video_tx, audio_buffer)
                .await
                .unwrap();
        });
    });

    video::run_window(&Window { width, height }, palette.as_ref(), &mut video_rx)?;

    CANCEL_TOKEN.cancel();
    Ok(())
}
