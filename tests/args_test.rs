#[cfg(test)]
mod tests {
    use clap::Parser;
    use lib::args::Args;
    use std::net::Ipv4Addr;

    #[test]
    fn test_custom_dimensions() {
        let args = Args::try_parse_from(&["program", "-d", "640x480"]).unwrap();
        assert_eq!(args.dimensions, (640, 480));
    }

    #[test]
    fn test_dimensions_long_form() {
        let args = Args::try_parse_from(&["program", "--dimensions", "1920x1080"]).unwrap();
        assert_eq!(args.dimensions, (1920, 1080));
    }

    #[test]
    fn test_mute_flag() {
        let args = Args::try_parse_from(&["program", "-m"]).unwrap();
        assert_eq!(args.mute, true);
    }

    #[test]
    fn test_mute_long_form() {
        let args = Args::try_parse_from(&["program", "--mute"]).unwrap();
        assert_eq!(args.mute, true);
    }

    #[test]
    fn test_palette() {
        let args = Args::try_parse_from(&[
            "program",
            "-p", "FF0000,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0"
        ]).unwrap();

        assert_eq!(args.palette.len(), 16);
        assert_eq!(args.palette[0], 0xFF0000);
        assert_eq!(args.palette[1], 0x00FF00);
        assert_eq!(args.palette[2], 0x0000FF);
        assert_eq!(args.palette[15], 0xC0C0C0);
    }

    #[test]
    fn test_palette_long_form() {
        let args = Args::try_parse_from(&[
            "program",
            "--palette", "000000,111111,222222,333333,444444,555555,666666,777777,888888,999999,AAAAAA,BBBBBB,CCCCCC,DDDDDD,EEEEEE,FFFFFF"
        ]).unwrap();

        assert_eq!(args.palette.len(), 16);
        assert_eq!(args.palette[0], 0x000000);
        assert_eq!(args.palette[15], 0xFFFFFF);
    }

    #[test]
    fn test_custom_video_multicast_address() {
        let args = Args::try_parse_from(&["program", "-v", "239.1.2.3"]).unwrap();
        assert_eq!(args.video_maddr, Ipv4Addr::new(239, 1, 2, 3));
    }

    #[test]
    fn test_custom_audio_multicast_address() {
        let args = Args::try_parse_from(&["program", "-a", "239.5.6.7"]).unwrap();
        assert_eq!(args.audio_maddr, Ipv4Addr::new(239, 5, 6, 7));
    }

    #[test]
    fn test_custom_video_port() {
        let args = Args::try_parse_from(&["program", "--video-port", "12000"]).unwrap();
        assert_eq!(args.video_port, 12000);
    }

    #[test]
    fn test_custom_audio_port() {
        let args = Args::try_parse_from(&["program", "--audio-port", "12001"]).unwrap();
        assert_eq!(args.audio_port, 12001);
    }

    #[test]
    fn test_invalid_multicast_address_not_multicast() {
        let result = Args::try_parse_from(&["program", "-v", "192.168.1.1"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not a multicast address"));
    }

    #[test]
    fn test_invalid_multicast_address_format() {
        let result = Args::try_parse_from(&["program", "-v", "999.999.999.999"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("parsing multicast address"));
    }

    #[test]
    fn test_invalid_multicast_address_malformed() {
        let result = Args::try_parse_from(&["program", "-a", "not.an.ip.address"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_flags_combined() {
        let args = Args::try_parse_from(&[
            "program",
            "-d", "800x600",
            "-m",
            "-p", "FF0000,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0",
            "-v", "239.10.20.30",
            "-a", "239.10.20.31",
            "--video-port", "15000",
            "--audio-port", "15001"
        ]).unwrap();

        assert_eq!(args.dimensions, (800, 600));
        assert_eq!(args.mute, true);
        assert_eq!(args.palette.len(), 16);
        assert_eq!(args.video_maddr, Ipv4Addr::new(239, 10, 20, 30));
        assert_eq!(args.audio_maddr, Ipv4Addr::new(239, 10, 20, 31));
        assert_eq!(args.video_port, 15000);
        assert_eq!(args.audio_port, 15001);
    }

    #[test]
    fn test_invalid_dimension_format() {
        let result = Args::try_parse_from(&["program", "-d", "640"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid format"));
    }

    #[test]
    fn test_invalid_dimension_separator() {
        let result = Args::try_parse_from(&["program", "-d", "640-480"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_dimensions_too_small() {
        let result = Args::try_parse_from(&["program", "-d", "100x100"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("too small"));
    }

    #[test]
    fn test_dimensions_too_large() {
        let result = Args::try_parse_from(&["program", "-d", "10000x10000"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("too large"));
    }

    #[test]
    fn test_dimensions_not_numbers() {
        let result = Args::try_parse_from(&["program", "-d", "abcxdef"]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid width") || err.contains("Invalid"));
    }

    #[test]
    fn test_palette_invalid_hex() {
        let result = Args::try_parse_from(&[
            "program",
            "-p",
            "GGGGGG,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0",
        ]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid hex value"));
    }

    #[test]
    fn test_palette_with_spaces() {
        let args = Args::try_parse_from(&[
            "program",
            "-p", "FF0000, 00FF00, 0000FF, FFFF00, FF00FF, 00FFFF, FFFFFF, 000000, 808080, 800000, 008000, 000080, 808000, 800080, 008080, C0C0C0"
        ]).unwrap();

        assert_eq!(args.palette.len(), 16);
        assert_eq!(args.palette[0], 0xFF0000);
        assert_eq!(args.palette[1], 0x00FF00);
    }

    #[test]
    fn test_palette_lowercase_hex() {
        let args = Args::try_parse_from(&[
            "program",
            "-p", "ff0000,00ff00,0000ff,ffff00,ff00ff,00ffff,ffffff,000000,808080,800000,008000,000080,808000,800080,008080,c0c0c0"
        ]).unwrap();

        assert_eq!(args.palette[0], 0xFF0000);
        assert_eq!(args.palette[15], 0xC0C0C0);
    }

    #[test]
    fn test_palette_mixed_case() {
        let args = Args::try_parse_from(&[
            "program",
            "-p", "Ff0000,00Ff00,0000Ff,FfFf00,Ff00Ff,00FfFf,FfFfFf,000000,808080,800000,008000,000080,808000,800080,008080,C0c0C0"
        ]).unwrap();

        assert_eq!(args.palette.len(), 16);
    }

    #[test]
    fn test_minimum_valid_dimensions() {
        let args = Args::try_parse_from(&["program", "-d", "320x200"]).unwrap();
        assert_eq!(args.dimensions, (320, 200));
    }

    #[test]
    fn test_maximum_valid_dimensions() {
        let args = Args::try_parse_from(&["program", "-d", "5120x3650"]).unwrap();
        assert_eq!(args.dimensions, (5120, 3650));
    }

    #[test]
    fn test_valid_multicast_ranges() {
        // Test various valid multicast addresses (224.0.0.0 to 239.255.255.255)
        let args1 = Args::try_parse_from(&["program", "-v", "224.0.0.1"]).unwrap();
        assert_eq!(args1.video_maddr, Ipv4Addr::new(224, 0, 0, 1));

        let args2 = Args::try_parse_from(&["program", "-v", "239.255.255.255"]).unwrap();
        assert_eq!(args2.video_maddr, Ipv4Addr::new(239, 255, 255, 255));
    }

    #[test]
    fn test_port_boundaries() {
        // Test minimum port
        let args1 = Args::try_parse_from(&["program", "--video-port", "1"]).unwrap();
        assert_eq!(args1.video_port, 1);

        // Test maximum port
        let args2 = Args::try_parse_from(&["program", "--audio-port", "65535"]).unwrap();
        assert_eq!(args2.audio_port, 65535);
    }

    #[test]
    fn test_invalid_port_too_large() {
        let result = Args::try_parse_from(&["program", "--video-port", "70000"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_long_form_multicast_addresses() {
        let args = Args::try_parse_from(&[
            "program",
            "--video-maddr",
            "239.100.100.100",
            "--audio-maddr",
            "239.200.200.200",
        ])
        .unwrap();

        assert_eq!(args.video_maddr, Ipv4Addr::new(239, 100, 100, 100));
        assert_eq!(args.audio_maddr, Ipv4Addr::new(239, 200, 200, 200));
    }
}
