# U64 Viewer

A real-time video and audio stream viewer for the Commodore 64 Ultimate cartridge.

## Overview

U64 Viewer receives and displays real-time video and audio streams multicast over the network from a Commodore 64 Ultimate cartridge. It decodes the C64's 4-bit indexed color video frames (384x272 pixels) and plays back the accompanying stereo audio at 48kHz, allowing you to watch and hear the C64's output on your computer screen.

## Features

- **Real-time video streaming**: Displays C64 video output at 384x272 resolution with 16-color palette
- **Audio playback**: Stereo audio streaming at ~48kHz with automatic buffer management
- **Customizable window size**: Adjust the display window to your preference
- **Custom color palettes**: Override the default C64 color palette with your own
- **Network flexibility**: Configure custom multicast addresses and ports for video and audio streams
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

- `--video-port <PORT>` - Custom video port number (default: 11000)
```bash
  u64-viewer --video-port 12000
```

- `--audio-port <PORT>` - Custom audio port number (default: 11001)
```bash
  u64-viewer --audio-port 12001
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

**Use custom ports:**
```bash
u64-viewer --video-port 12000 --audio-port 12001
```

**Full customization:**
```bash
u64-viewer \
  --dimensions 1024x768 \
  --video-maddr 239.10.20.30 \
  --audio-maddr 239.10.20.31 \
  --video-port 15000 \
  --audio-port 15001 \
  --palette FF0000,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0
```

## Network Configuration

The viewer listens for UDP packets on:
- **Video**: 239.0.1.64:11000 (configurable via `--video-maddr` and `--video-port`)
- **Audio**: 239.0.1.65:11001 (configurable via `--audio-maddr` and `--audio-port`)

### Multicast vs Unicast

**Multicast (Default):**
The viewer joins multicast groups to receive streams. This allows multiple viewers to receive the same stream simultaneously without additional network overhead.

**Unicast:**
You can also use unicast by configuring the C64 Ultimate to send directly to the IP address of the machine running u64-viewer. To use unicast:

1. On the C64 Ultimate, configure the stream destination to your computer's IP address (e.g., 192.168.1.100)
2. Set the video and audio ports on the Ultimate to match the ports u64-viewer is listening on
3. Start u64-viewer with the same ports (default 11000 for video, 11001 for audio):
```bash
   u64-viewer
```
   Or with custom ports:
```bash
   u64-viewer --video-port 12000 --audio-port 12001
```

**Note:** When using unicast, the `--video-maddr` and `--audio-maddr` options are ignored since the viewer binds to `0.0.0.0` (all interfaces) and doesn't join multicast groups. Only the port numbers matter for unicast.

Ensure your firewall allows UDP traffic on these ports and that your network supports multicast (for multicast mode).

### Multicast Address Requirements

- Must be a valid IPv4 multicast address (224.0.0.0 to 239.255.255.255)
- Common ranges:
  - **224.0.0.0 - 224.0.0.255**: Reserved for local network control
  - **239.0.0.0 - 239.255.255.255**: Administratively scoped (organization-local)

### Port Requirements

- Valid port range: 1-65535
- Avoid well-known ports (1-1023) unless you have appropriate permissions
- Default ports (11000, 11001) are in the registered port range

### Protocol Details

**Video Stream:**
- Each packet contains 4 lines of video data (768 bytes)
- Frame format: 384 pixels wide, variable height
- Color depth: 4 bits per pixel (16 colors)
- Packet size: 780 bytes (12 byte header + 768 bytes data)
- Header format:
  - Sequence number (2 bytes)
  - Frame number (2 bytes)
  - Line number (2 bytes, bit 15 = end-of-frame marker)
  - Width (2 bytes)
  - Lines per packet (1 byte)
  - Bits per pixel (1 byte)
  - Encoding type (2 bytes)

**Audio Stream:**
- Sample rate: ~47983 Hz (PAL) or configurable
- Format: 16-bit signed stereo (interleaved)
- 192 stereo samples per packet (384 samples total)
- Packet size: 770 bytes (2 byte header + 768 bytes data)
- Header format:
  - Sequence number (2 bytes)

## Keyboard Controls

- **ESC** - Exit the viewer

## Troubleshooting

### No video appears

1. Verify the C64 Ultimate is streaming to the correct multicast address
2. Check firewall settings allow UDP multicast traffic
3. Ensure you're on the same network segment as the C64 Ultimate
4. Verify the multicast address and port match the Ultimate's configuration
5. Try running with `RUST_LOG=debug` environment variable for diagnostic output:
```bash
   RUST_LOG=debug u64-viewer
```

### Audio is choppy or distorted

- The audio uses a ring buffer with pre-buffering. If you experience issues, try:
  - Checking network stability (packet loss can cause audio gaps)
  - Ensuring your system isn't under heavy CPU load
  - Verifying sample rate compatibility with your audio device
  - Looking for "Dropped audio packet" messages in debug logs

### Window doesn't respond

- Make sure you're not setting dimensions that are too large for your display
- Valid range: 320x200 minimum, 5120x3650 maximum

### Multicast not working

- Ensure your router/switch supports IGMP (Internet Group Management Protocol)
- Check that multicast is not blocked by your firewall
- **Try unicast instead**: Configure the C64 Ultimate to send directly to your computer's IP address
- On Linux, you may need to add a multicast route:
```bash
  sudo route add -net 224.0.0.0 netmask 240.0.0.0 dev eth0
```
- On Windows, check Windows Defender Firewall allows UDP on the specified ports

### Custom multicast addresses rejected

- Ensure the address is in the valid multicast range (224.0.0.0 - 239.255.255.255)
- Non-multicast addresses (e.g., 192.168.x.x) will be rejected with an error

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

tests/
├── audio_ringbuffer_test.rs  # Ring buffer tests
├── protocol_test.rs           # Protocol parsing tests
├── constants_test.rs          # Color conversion tests
├── integration_test.rs        # Integration tests
└── args_test.rs               # CLI argument tests
```

### Running tests
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test args_test
cargo test audio_ringbuffer_test

# Run with output
cargo test -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test
```

### Building with optimizations
```bash
cargo build --release
```

### Debug logging

Set the `RUST_LOG` environment variable to control logging levels:
```bash
# Show all debug messages
RUST_LOG=debug u64-viewer

# Show only info and above
RUST_LOG=info u64-viewer

# Show only warnings and errors
RUST_LOG=warn u64-viewer
```

## Dependencies

- **clap** - Command-line argument parsing with validation
- **tokio** - Async runtime for network I/O
- **tokio-util** - Cancellation token for graceful shutdown
- **minifb** - Cross-platform window and framebuffer
- **cpal** - Cross-platform audio I/O
- **zerocopy** - Zero-copy parsing of network packets
- **tracing** - Structured logging and diagnostics

## Performance Considerations

- **Audio buffer**: Pre-buffers 0.25 seconds (12,000 samples) before playback starts to prevent underruns
- **Video channel**: Buffers up to 20 frames to handle network jitter
- **CPU usage**: Minimal - uses async I/O for network operations
- **Memory usage**: Small fixed buffers for audio and video data

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Contribution Guidelines

- Write tests for new features
- Maintain existing code style
- Update documentation for user-facing changes
- Ensure all tests pass before submitting

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
- [Ultimate 64 Network Stream Documentation](https://github.com/GideonZ/1541ultimate/tree/master/software/network)
- [Report Issues](https://github.com/yourusername/u64-viewer/issues)

## FAQ

**Q: Can I use this with multiple C64 systems simultaneously?**
A: Yes! With multicast, configure each C64 to use different multicast addresses and run multiple instances of u64-viewer with the appropriate `--video-maddr` and `--audio-maddr` options. With unicast, configure each C64 to use different port numbers and run multiple viewers with different `--video-port` and `--audio-port` settings.

**Q: Should I use multicast or unicast?**
A:
- **Use multicast** if you want multiple viewers to watch the same stream simultaneously, or if your C64 Ultimate is already configured for multicast
- **Use unicast** if you only need one viewer, want to avoid multicast routing complexity, or your network doesn't support multicast properly

**Q: Does this work over the internet?**
A: Multicast typically doesn't work over the internet. For internet streaming, configure the C64 Ultimate to use unicast with your public IP address (and set up appropriate port forwarding on your router).

**Q: What's the latency?**
A: Typically 250-500ms depending on network conditions and the audio pre-buffer settings.

**Q: Can I record the stream?**
A: Currently no, but this would be a great feature for a future release!

**Q: My network doesn't support multicast, what should I do?**
A: Configure your C64 Ultimate to use unicast mode by setting the destination IP to your computer's address. The viewer will work the same way, just ensure the ports match.

---

Made with ❤️ for the C64 community
