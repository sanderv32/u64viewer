pub mod args;
pub mod audio;
pub mod video;
pub mod constants;
pub mod ringbuffer;
pub mod network;

pub use ringbuffer::RingBuffer;
pub use network::{NetworkConfig, network_tasks};
pub use audio::{AudioBuffer, init_audio};
pub use video::{Window, run_window};
pub use constants::{WIDTH, HEIGHT, COLORS, colors_to_u32};

use std::sync::LazyLock;
use tokio_util::sync::CancellationToken;

pub static CANCEL_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
