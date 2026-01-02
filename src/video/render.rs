use minifb::{Key, WindowOptions};
use tokio::sync::mpsc::Receiver;

use crate::constants::{COLORS, HEIGHT, WIDTH, colors_to_u32};

pub struct Window {
    pub width: usize,
    pub height: usize,
}

/// # Panics
/// Panics if there are conversion errors
/// # Errors
/// Returns an error if unable to open the window
pub fn run_window(
    window: &Window,
    palette: Option<&Vec<u32>>,
    video_rx: &mut Receiver<Vec<u8>>,
) -> Result<(), String> {
    let mut window = minifb::Window::new(
        "U64 Viewer - ESC to exit",
        window.width,
        window.height,
        WindowOptions {
            resize: true,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    )
    .map_err(|e| format!("ERROR: {e}"))?;

    let mut frame = vec![0u32; WIDTH * HEIGHT].into_boxed_slice();
    let colors: [[u8; 4]; 16] = if let Some(palette) = palette
        && !palette.is_empty()
    {
        let mut result = [[0u8; 4]; 16];
        for (i, &color) in palette.iter().enumerate() {
            // Extract RGB from hex color (assuming format 0xRRGGBB)
            let r = u8::try_from((color >> 16) & 0xFF).expect("conversion error");
            let g = u8::try_from((color >> 8) & 0xFF).expect("conversion error");
            let b = u8::try_from(color & 0xFF).expect("conversion error");
            result[i] = [0, r, g, b]; // ARGB format with alpha = 0
        }
        result
    } else {
        COLORS
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        while let Ok(video_buffer) = video_rx.try_recv() {
            let mut pos = 0;
            for y_pos in 0..video_buffer.len() {
                for x_pos in 0..192 {
                    if pos >= video_buffer.len() {
                        break;
                    }
                    let byte = video_buffer[pos] as usize;
                    let frame_pos = (y_pos * WIDTH) + 2 * x_pos;
                    if frame_pos >= frame.len() {
                        continue;
                    }
                    frame[frame_pos] = colors_to_u32(colors[byte & 0x0f]);
                    frame[frame_pos + 1] = colors_to_u32(colors[byte >> 4]);
                    pos += 1;
                }
            }
        }
        _ = window.update_with_buffer(&frame, WIDTH, HEIGHT);
        // TODO: Do we really need this sleep?
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}
