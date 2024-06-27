use core::sync::atomic::{AtomicPtr, Ordering};

use crate::bootinfo::FRAMEBUFFER_REQUEST;
use crate::framebuffer::chars::{get_char_bitmap, FONT_HEIGHT, FONT_WIDTH};
// External crate for bootloader-specific functions and types.
extern crate limine;
use limine::framebuffer::Framebuffer;
use spin::lazy::Lazy;
use spin::mutex::TicketMutex;

use super::console::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

/// Global access to the framebuffer
pub static FRAMEBUFFER: Lazy<TicketMutex<FrameBufferInfo>> =
    Lazy::new(|| TicketMutex::new(init_framebuffer()));

/// A struct representing the framebuffer information,
/// including its memory address, dimensions, pixel format, etc.
pub struct FrameBufferInfo {
    address: AtomicPtr<u32>,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
    scale: u8,
}

/// including its memory address, dimensions, pixel format, etc.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[allow(unused)]
impl FrameBufferInfo {
    /// Constructs a new `FrameBufferInfo` instance from a limine framebuffer.
    ///
    /// # Arguments
    ///
    /// * `framebuffer` - A reference to a limine `Framebuffer` struct.
    pub fn new(framebuffer: &Framebuffer) -> Self {
        /* ARM safeguard */
        assert!(framebuffer.addr() as usize % 4 == 0);

        let mut framebuffer = Self {
            #[allow(clippy::cast_ptr_alignment)]
            address: AtomicPtr::new(framebuffer.addr().cast::<u32>()),
            width: framebuffer.width(),
            height: framebuffer.height(),
            pitch: framebuffer.pitch(),
            bpp: framebuffer.bpp(),
            scale: 1,
        };

        /* Initialize framebuffer scale automatically */
        framebuffer.calc_scale();
        framebuffer
    }

    /// Draws a line between two points using Bresenham's line algorithm.
    ///
    /// # Arguments
    ///
    /// * `p0` - The start point of the line.
    /// * `p1` - The end point of the line.
    /// * `color` - The color of the line in ARGB format.
    pub fn draw_line(&self, p0: Point, p1: Point, color: u32) {
        let mut x0 = p0.x;
        let mut y0 = p0.y;
        let x1 = p1.x;
        let y1 = p1.y;

        let dx = x1.abs_diff(x0);
        let dy = y1.abs_diff(y0);
        let mut err = dx + dy; // error value e_xy

        loop {
            self.draw_pixel(x0, y0, color);
            // Draw the current pixel
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                // e_xy+e_x > 0
                err += dy;
                if x0 < x1 {
                    x0 += 1
                } else {
                    x0 -= 1
                };
            }
            if e2 <= dx {
                // e_xy+e_y < 0
                err += dx;
                if y0 < y1 {
                    y0 += 1
                } else {
                    y0 -= 1
                };
            }
        }
    }

    /// Clears the entire screen to a single color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to fill the screen with, in ARGB format.
    pub fn clear_screen(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_offset = (y * self.pitch / u64::from(self.bpp / 8) + x) as usize;
                unsafe {
                    *self.address.load(Ordering::Relaxed).add(pixel_offset) = color;
                }
            }
        }
    }

    /// Draws a single pixel at the specified location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    /// * `color` - The color of the pixel in ARGB format.

    pub fn draw_pixel(&self, x: u64, y: u64, color: u32) {
        if x < self.width && y < self.height {
            let pixel_offset = (y * self.pitch / u64::from(self.bpp / 8) + x) as usize;
            unsafe {
                *self.address.load(Ordering::Relaxed).add(pixel_offset) = color;
            }
        }
    }

    /// Draws text starting from a specified location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the starting point of the text.
    /// * `y` - The y coordinate of the starting point of the text.
    /// * `text` - The text to draw.
    /// * `color` - The color of the text in ARGB format.

    pub fn draw_text(&self, mut x: u64, mut y: u64, text: &str, color: u32, background_color: u32) {
        let start_x = x; // Remember the starting x position to reset to it on new lines
        for c in text.chars() {
            if c == '\n' {
                y += FONT_HEIGHT as u64 * u64::from(self.scale) + 1;
                x = start_x;
            } else {
                self.draw_char(x, y, c, color, background_color);
                x += FONT_WIDTH as u64 * u64::from(self.scale);
            }
        }
    }

    /// Helper method to draw a single character from its bitmap.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate where the character should be drawn.
    /// * `y` - The y coordinate where the character should be drawn.
    /// * `bitmap` - A reference to the bitmap array representing the character.
    /// * `color` - The color of the character in ARGB format.
    pub fn draw_char(&self, x: u64, y: u64, chracter: char, color: u32, background_color: u32) {
        let bitmap = get_char_bitmap(chracter);
        for (row, &bits) in bitmap.iter().enumerate() {
            for col in 0..FONT_WIDTH as u64 {
                let is_set = (bits >> (FONT_WIDTH as u64 - 1 - col)) & 1 == 1;
                let pixel_color = if is_set { color } else { background_color };
                /* Instead of a pixel, create a square with sides that are the size of self.scale */
                for dy in 0..self.scale {
                    for dx in 0..self.scale {
                        self.draw_pixel(
                            x + col * u64::from(self.scale) + u64::from(dx),
                            y + row as u64 * u64::from(self.scale) + u64::from(dy),
                            pixel_color,
                        );
                    }
                }
            }
        }
    }

    /// Draws a rectangle at the specified location and dimensions.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the rectangle's top-left corner.
    /// * `y` - The y coordinate of the rectangle's top-left corner.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    /// * `color` - The color of the rectangle in ARGB format.
    pub fn draw_rect(&self, x: u64, y: u64, width: u64, height: u64, color: u32) {
        for row in y..y + height {
            for col in x..x + width {
                self.draw_pixel(col, row, color);
            }
        }
    }

    /// Draws a filled triangle between three points.
    ///
    /// # Arguments
    ///
    /// * `p1`, `p2`, `p3` - The vertices of the triangle.
    /// * `color` - The color to fill the triangle with, in ARGB format.
    pub fn draw_triangle(&self, p1: Point, p2: Point, p3: Point, color: u32) {
        // First, sort vertices by y-coordinate
        let mut vertices = [p1, p2, p3];
        vertices.sort_unstable_by_key(|v| v.y);

        // Define a closure to interpolate x values for a given y
        let interpolate_x = |p1: Point, p2: Point, current_y: u64| -> u64 {
            if p1.y == p2.y {
                return p1.x;
            }
            p1.x + (p2.x - p1.x) * (current_y - p1.y) / (p2.y - p1.y)
        };

        // Function to fill between two edges from startY to endY
        let fill_between_edges = |self_ref: &Self,
                                  start_y: u64,
                                  end_y: u64,
                                  p_left: Point,
                                  p_right_start: Point,
                                  p_right_end: Point| {
            for y in start_y..=end_y {
                let x_start = interpolate_x(p_left, p_right_start, y);
                let x_end = interpolate_x(p_left, p_right_end, y);
                for x in x_start.min(x_end)..=x_start.max(x_end) {
                    self_ref.draw_pixel(x, y, color);
                }
            }
        };

        // Fill from top vertex to middle vertex
        fill_between_edges(
            self,
            vertices[0].y,
            vertices[1].y,
            vertices[0],
            vertices[1],
            vertices[2],
        );

        // Fill from middle vertex to bottom vertex
        fill_between_edges(
            self,
            vertices[1].y,
            vertices[2].y,
            vertices[2],
            vertices[0],
            vertices[1],
        );
    }

    /// Return the framebuffer scaling multiplier
    pub fn get_scale(&self) -> u8 {
        self.scale
    }

    /// Automatically select scaling based on resolution
    /// Call this whenever resolution of the monitor changes!
    pub fn calc_scale(&mut self) {
        let scale_width = self.width / (CONSOLE_WIDTH * FONT_WIDTH) as u64;
        let scale_height = self.height / (CONSOLE_HEIGHT * FONT_HEIGHT) as u64;
        self.scale = u8::try_from(if (scale_height > scale_width) {
            scale_width
        } else {
            scale_height
        })
        .expect("Framebuffer scale > 255");
    }
}

/// Initializes the framebuffer and returns a `FrameBufferInfo` instance if successful.
pub fn init_framebuffer() -> FrameBufferInfo {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if framebuffer_response.framebuffers().count() < 1 {
            panic!("No framebuffer returned from bootloader!");
        }

        let framebuffer = &framebuffer_response.framebuffers().next().unwrap();
        FrameBufferInfo::new(framebuffer)
    } else {
        panic!("No framebuffer returned from bootlaoder!");
    }
}
