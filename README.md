# U64 Viewer

A real-time video and audio stream viewer for the Commodore 64 Ultimate cartridge.

## Overview

U64 Viewer receives and displays real-time video and audio streams multicast over the network from a Commodore 64 Ultimate cartridge. It decodes the C64's 4-bit indexed color video frames (384x272 pixels) and plays back the accompanying stereo audio at 48kHz, allowing you to watch and hear the C64's output on your computer screen.

## Features

- **Real-time video streaming**: Displays C64 video output at 384x272 resolution with 16-color palette
- **Audio playback**: Stereo audio streaming at ~48kHz with automatic buffer management
- **Customizable window size**: Adjust the display window to your preference
- **Custom color palettes**: Override the default C64 color palette with your own
- **Network flexibility**: Configure custom multicast addresses for video and audio streams
- **Low latency**: Efficient ring buffer implementation for smooth audio playback

## Installation

### Prerequisites

- Rust 1.70 or later
- A Commodore 64 Ultimate cartridge configured to stream video/audio

### Building from source
```bash
git clone https://github.com/yourusername/u64-viewer.git
cd u64-viewer
cargo build --release
```

The compiled binary will be available at `target/release/u64-viewer`.

## Usage

### Basic usage
```bash
u64-viewer
```

This will start the viewer with default settings:
- Window size: 384x272
- Video multicast address: 239.0.1.64:11000
- Audio multicast address: 239.0.1.65:11001

### Command-line options
```bash
u64-viewer [OPTIONS]
```

#### Options

- `-d, --dimensions <WIDTHxHEIGHT>` - Set window dimensions (default: 384x272)
```bash
  u64-viewer -d 768x544
```

- `-m, --mute` - Disable audio playback
```bash
  u64-viewer --mute
```

- `-p, --palette <HEX_COLORS>` - Use custom RGB palette (16 comma-separated hex values)
```bash
  u64-viewer -p FF0000,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0
```

- `-v, --video-maddr <ADDRESS>` - Custom video multicast address (default: 239.0.1.64)
```bash
  u64-viewer -v 239.0.2.64
```

- `-a, --audio-maddr <ADDRESS>` - Custom audio multicast address (default: 239.0.1.65)
```bash
  u64-viewer -a 239.0.2.65
```

- `-h, --help` - Display help information

### Examples

**Double the window size:**
```bash
u64-viewer --dimensions 768x544
```

**Mute audio and use custom palette:**
```bash
u64-viewer --mute --palette F0F0F0,000000,8D2F34,6AD4CD,9835A4,4CB442,2C29B1,A09000,984E20,5B3800,D1676D,999999,666666,20A020,2020A0,333333
```

**Use custom multicast addresses:**
```bash
u64-viewer --video-maddr 239.1.1.100 --audio-maddr 239.1.1.101
```

## Network Configuration

The viewer listens for multicast UDP packets on:
- **Video**: 239.0.1.64:11000 (configurable)
- **Audio**: 239.0.1.65:11001 (configurable)

Ensure your firewall allows UDP traffic on these ports and that your network supports multicast.

### Protocol Details

**Video Stream:**
- Each packet contains 4 lines of video data (768 bytes)
- Frame format: 384 pixels wide, variable height
- Color depth: 4 bits per pixel (16 colors)
- Packet size: 780 bytes (12 byte header + 768 bytes data)

**Audio Stream:**
- Sample rate: ~47983 Hz (PAL) or configurable
- Format: 16-bit signed stereo (interleaved)
- 192 stereo samples per packet (384 samples total)
- Packet size: 770 bytes (2 byte header + 768 bytes data)

## Keyboard Controls

- **ESC** - Exit the viewer

## Troubleshooting

### No video appears

1. Verify the C64 Ultimate is streaming to the correct multicast address
2. Check firewall settings allow UDP multicast traffic
3. Ensure you're on the same network segment as the C64 Ultimate
4. Try running with `RUST_LOG=debug` environment variable for diagnostic output:
```bash
   RUST_LOG=debug u64-viewer
```

### Audio is choppy or distorted

- The audio uses a ring buffer with pre-buffering. If you experience issues, try:
  - Checking network stability (packet loss can cause audio gaps)
  - Ensuring your system isn't under heavy CPU load
  - Verifying sample rate compatibility with your audio device

### Window doesn't respond

- Make sure you're not setting dimensions that are too large for your display
- Valid range: 320x200 minimum, 5120x3650 maximum

## Development

### Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── args.rs              # Command-line argument parsing
├── constants.rs         # Color palettes and constants
├── audio/
│   ├── mod.rs          # Audio module
│   ├── ringbuffer.rs   # Ring buffer implementation
│   └── stream.rs       # Audio initialization
├── video/
│   ├── mod.rs          # Video module
│   └── render.rs       # Window rendering
└── network/
    ├── mod.rs          # Network module
    └── protocol.rs     # Protocol handlers
```

### Running tests
```bash
cargo test
```

### Building with optimizations
```bash
cargo build --release
```

## Dependencies

- **clap** - Command-line argument parsing
- **tokio** - Async runtime for network I/O
- **minifb** - Cross-platform window and framebuffer
- **cpal** - Cross-platform audio I/O
- **zerocopy** - Zero-copy parsing of network packets
- **tracing** - Structured logging

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see below for details:
```
MIT License

Copyright (c) 2024 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Acknowledgments

- Thanks to the Ultimate 64/Ultimate 1541 development team for creating the streaming protocol
- Inspired by the Commodore 64 retro computing community

## Links

- [Commodore 64 Ultimate Cartridge](https://ultimate64.com/)
- [Report Issues](https://github.com/yourusername/u64-viewer/issues)

---

Made with ❤️ for the C64 community
