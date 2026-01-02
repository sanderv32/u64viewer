use clap::Parser;
use std::{net::Ipv4Addr, str::FromStr};

/// C64 Ultimate Stream viewer
///
/// This viewer receives and displays real-time video and audio streams multicast over the
/// network from a Commodore 64 Ultimate cartridge. It decodes the C64's 4-bit indexed color
/// video frames (384x272 pixels) and plays back the accompanying stereo audio at 48kHz,
/// allowing you to watch and hear the C64's output on your computer screen.
#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Window dimension (e.g. 320x200, 640x480)
    #[arg(short, long, value_parser = parse_dimensions, default_value = "384x272")]
    pub dimensions: (usize, usize),
    /// Mute audio
    #[arg(short, long, default_value_t = false)]
    pub mute: bool,
    /// Alternate RGB palette
    #[arg(short, long, value_parser = parse_palette, value_delimiter = ',')]
    pub palette: Vec<u32>,
    /// Use alternate multicast address for video
    #[arg(short, long, value_parser = parse_maddr, default_value = "239.0.1.64")]
    pub video_maddr: Ipv4Addr,
    /// Use alternate multicast address for audio
    #[arg(short, long, value_parser = parse_maddr, default_value = "239.0.1.65")]
    pub audio_maddr: Ipv4Addr,
    /// Use alternate port number for video
    #[arg(long, default_value_t = 11_000)]
    pub video_port: u16,
    /// Use alternate port number for audio
    #[arg(long, default_value_t = 11_001)]
    pub audio_port: u16,
}

fn parse_dimensions(s: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = s.split('x').collect();

    if parts.len() != 2 {
        return Err(format!(
            "Invalid format '{s}'. Expected format: WIDTHxHEIGHT"
        ));
    }

    let width = parts[0]
        .parse::<usize>()
        .map_err(|_| format!("Invalid width: {}", parts[0]))?;
    let height = parts[1]
        .parse::<usize>()
        .map_err(|_| format!("Invalid height: {}", parts[1]))?;

    if width < 320 || height < 200 {
        return Err("Dimensions too small (min 320x200)".to_string());
    }

    if width > 5120 || height > 3650 {
        return Err("Dimensions too large (max 5120x3650)".to_string());
    }

    Ok((width, height))
}

fn parse_palette(s: &str) -> Result<u32, String> {
    let trimmed = s.trim();
    u32::from_str_radix(trimmed, 16).map_err(|_| format!("Invalid hex value: '{s}'."))
}

fn parse_maddr(addr: &str) -> Result<Ipv4Addr, String> {
    let addr: Ipv4Addr = Ipv4Addr::from_str(addr)
        .map_err(|_| format!("Error parsing multicast address : {addr}"))?;
    if !addr.is_multicast() {
        return Err(format!("Address {addr} is not a multicast address"));
    }
    Ok(addr)
}
