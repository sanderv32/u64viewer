mod protocol;

use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;
use tracing::debug;

use crate::{AudioBuffer, CANCEL_TOKEN};

pub struct NetworkConfig {
    pub video_maddr: Ipv4Addr,
    pub audio_maddr: Ipv4Addr,
    pub video_port: u16,
    pub audio_port: u16,
}

/// # Errors
/// Returns an error if unable to bind to socket
pub async fn network_tasks(
    config: NetworkConfig,
    video_tx: Sender<Vec<u8>>,
    audio_buffer: Option<AudioBuffer>, // If muted `audio_buffer` is `None`
) -> Result<(), String> {
    debug!("Setting up network tasks");
    let video_maddr = config.video_maddr;
    let audio_maddr = config.audio_maddr;
    let video_port = config.video_port;
    let audio_port = config.audio_port;
    let video_socket = UdpSocket::bind(format!("0.0.0.0:{video_port}"))
        .await
        .map_err(|e| e.to_string())?;
    video_socket
        .join_multicast_v4(video_maddr, Ipv4Addr::UNSPECIFIED)
        .map_err(|e| e.to_string())?;
    let video_task =
        tokio::spawn(async move { protocol::handle_video(video_socket, video_tx).await });

    let audio_task = if let Some(audio_buffer) = audio_buffer {
        let audio_socket = UdpSocket::bind(format!("0.0.0.0:{audio_port}"))
            .await
            .map_err(|e| e.to_string())?;
        audio_socket
            .join_multicast_v4(audio_maddr, Ipv4Addr::UNSPECIFIED)
            .map_err(|e| e.to_string())?;
        tokio::spawn(async move { protocol::handle_audio(audio_socket, audio_buffer).await })
    } else {
        // Audio is muted
        tokio::spawn(async move {
            CANCEL_TOKEN.cancelled().await;
            Ok(())
        })
    };

    // Wait for both tasks
    _ = tokio::try_join!(video_task, audio_task)
      .map_err(|e| format!("Task join error: {e}"))?;

    Ok(())
}
