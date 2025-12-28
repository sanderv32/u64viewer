mod protocol;

use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::AudioBuffer;

pub async fn network_tasks(
    video_tx: Sender<Vec<u8>>,
    audio_buffer: Option<AudioBuffer>,
) -> Result<(), String> {
    let video_socket = UdpSocket::bind("0.0.0.0:11000").await
        .map_err(|e| e.to_string())?;
    video_socket.join_multicast_v4(Ipv4Addr::new(239, 0, 1, 64), Ipv4Addr::UNSPECIFIED)
        .map_err(|e| e.to_string())?;
    let video_task =
        tokio::spawn(async move { protocol::handle_video(video_socket, video_tx).await });

    let audio_task = if let Some(audio_buffer) = audio_buffer {
        let audio_socket = UdpSocket::bind("0.0.0.0:11001").await
            .map_err(|e| e.to_string())?;
        audio_socket.join_multicast_v4(Ipv4Addr::new(239, 0, 1, 65), Ipv4Addr::UNSPECIFIED)
            .map_err(|e| e.to_string())?;
        tokio::spawn(async move { protocol::handle_audio(audio_socket, audio_buffer).await })
    } else {
        let token = CancellationToken::new();
        let token_clone = token.clone();
        tokio::spawn(async move {
            token_clone.cancelled().await;
            Ok(())
        })
    };

    // Wait for both tasks
    let _ = tokio::try_join!(video_task, audio_task);
    Ok(())
}
