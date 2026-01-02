use lib::constants::{COLORS, colors_to_u32};

#[test]
fn test_colors_array_size() {
    assert_eq!(COLORS.len(), 16);
}

#[test]
fn test_color_format() {
    // Each color should be ARGB format
    for color in &COLORS {
        assert_eq!(color.len(), 4);
    }
}

#[test]
fn test_colors_to_u32_black() {
    let black = [0x00, 0x00, 0x00, 0x00];
    assert_eq!(colors_to_u32(black), 0x00000000);
}

#[test]
fn test_colors_to_u32_white() {
    let white = [0x00, 0xFF, 0xFF, 0xFF];
    assert_eq!(colors_to_u32(white), 0x00FFFFFF);
}

#[test]
fn test_colors_to_u32_red() {
    let red = [0x00, 0xFF, 0x00, 0x00];
    assert_eq!(colors_to_u32(red), 0x00FF0000);
}

#[test]
fn test_colors_to_u32_arbitrary() {
    let color = [0x12, 0x34, 0x56, 0x78];
    let expected = (0x12 << 24) | (0x34 << 16) | (0x56 << 8) | 0x78;
    assert_eq!(colors_to_u32(color), expected);
}
