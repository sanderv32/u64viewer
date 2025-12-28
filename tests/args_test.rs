#[cfg(test)]
mod tests {
    use clap::Parser;
    use lib::args::Args;

    #[test]
    fn test_default_args() {
        let args = Args::try_parse_from(&["program"]).unwrap();
        assert_eq!(args.dimensions, (384, 272));
        assert_eq!(args.mute, false);
        assert!(args.palette.is_empty());
    }

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
    fn test_all_flags_combined() {
        let args = Args::try_parse_from(&[
            "program",
            "-d", "800x600",
            "-m",
            "-p", "FF0000,00FF00,0000FF,FFFF00,FF00FF,00FFFF,FFFFFF,000000,808080,800000,008000,000080,808000,800080,008080,C0C0C0"
        ]).unwrap();

        assert_eq!(args.dimensions, (800, 600));
        assert_eq!(args.mute, true);
        assert_eq!(args.palette.len(), 16);
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
}
