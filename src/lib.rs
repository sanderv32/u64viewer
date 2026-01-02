pub mod args;
pub mod audio;
pub mod video;
pub mod constants;
pub mod ringbuffer;
pub mod network;

pub use ringbuffer::RingBuffer;
pub use network::*;
pub use audio::*;
pub use video::*;
pub use constants::*;

use std::sync::LazyLock;
use tokio_util::sync::CancellationToken;

pub static CANCEL_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
