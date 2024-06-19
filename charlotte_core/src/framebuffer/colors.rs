/// Define as a `non_exhaustive` struct to behave as an "enum" with constant values
/// Doing it this way avoids explicit type casting with Rust enums
#[non_exhaustive]
pub struct Color;

#[allow(unused)]
impl Color {
    pub const BLACK: u32 = 0x00000000;
    pub const WHITE: u32 = 0xFFFFFFFF;
    pub const RED: u32 = 0x00FF0000;
    pub const GREEN: u32 = 0x0000FF00;
    pub const BLUE: u32 = 0x000000FF;
    pub const YELLOW: u32 = 0x00FFFF00;
    pub const MAGENTA: u32 = 0x00FF00FF;
    pub const CYAN: u32 = 0x0000FFFF;
}

#[allow(unused)]
pub fn blend_colors(foreground: u32, background: u32, blend_factor: u8) -> u32 {
    let fg_ratio = u32::from(blend_factor);
    let bg_ratio = 255 - fg_ratio;

    let r = (((foreground >> 16) & 0xFF) * fg_ratio + ((background >> 16) & 0xFF) * bg_ratio) / 255;
    let g = (((foreground >> 8) & 0xFF) * fg_ratio + ((background >> 8) & 0xFF) * bg_ratio) / 255;
    let b = ((foreground & 0xFF) * fg_ratio + (background & 0xFF) * bg_ratio) / 255;

    (r << 16) | (g << 8) | b
}
