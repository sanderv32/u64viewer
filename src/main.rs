use clap::Parser;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc::{self};

use lib::{CANCEL_TOKEN, RingBuffer, args::Args, video::Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let (width, height) = args.dimensions;
    if !args.palette.is_empty() && args.palette.len() != 16 {
        eprintln!(
            "Error: Palette must have exactly 16 colors, got {}",
            args.palette.len()
        );
        std::process::exit(1);
    }

    let palette = (!args.palette.is_empty()).then_some(args.palette);

    let (audio_buffer, _stream) = if args.mute {
        (None, None)
    } else {
        let buffer = Arc::new(Mutex::new(RingBuffer::new(48_000, 12_000)));
        let stream = lib::init_audio(&buffer.clone());
        (Some(buffer), Some(stream))
    };

    // Create channel for video buffer updates
    let (video_tx, mut video_rx) = mpsc::channel::<Vec<u8>>(20);

    // Spawn concurrent tasks
    let network_config = lib::NetworkConfig {
        video_maddr: args.video_maddr,
        audio_maddr: args.audio_maddr,
        video_port: args.video_port,
        audio_port: args.audio_port,
    };
    thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Failed to create tokio runtime: {e}");
                return;
            }
        };
        rt.block_on(async {
            if let Err(e) = lib::network_tasks(network_config, video_tx, audio_buffer).await {
                eprintln!("Network task error: {e}");
            }
        });
    });

    lib::run_window(&Window { width, height }, palette.as_deref(), &mut video_rx)?;

    CANCEL_TOKEN.cancel();
    Ok(())
}
