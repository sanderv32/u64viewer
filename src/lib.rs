pub mod args;
pub mod audio;
pub mod constants;
pub mod network;
pub mod ringbuffer;
pub mod video;

pub use audio::{AudioBuffer, init_audio};
pub use constants::{COLORS, HEIGHT, WIDTH, colors_to_u32};
pub use network::{NetworkConfig, network_tasks};
pub use ringbuffer::RingBuffer;
pub use video::{Window, run_window};

use std::sync::LazyLock;
use tokio_util::sync::CancellationToken;

pub static CANCEL_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
