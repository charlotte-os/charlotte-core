pub fn blend_colors(foreground: u32, background: u32, blend_factor: u8) -> u32 {
    let fg_ratio = blend_factor as u32;
    let bg_ratio = 255 - fg_ratio as u32;

    let r = (((foreground >> 16) & 0xFF) * fg_ratio + ((background >> 16) & 0xFF) * bg_ratio) / 255;
    let g = (((foreground >> 8) & 0xFF) * fg_ratio + ((background >> 8) & 0xFF) * bg_ratio) / 255;
    let b = ((foreground & 0xFF) * fg_ratio + (background & 0xFF) * bg_ratio) / 255;

    (r << 16) | (g << 8) | b
}