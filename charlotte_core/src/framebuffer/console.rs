use core::fmt;
use spin::mutex::TicketMutex;

use crate::framebuffer::{
    chars::{FONT_HEIGHT, FONT_WIDTH},
    framebuffer::FRAMEBUFFER,
};

const CONSOLE_WIDTH: usize = 80;
const CONSOLE_HEIGHT: usize = 25;

pub static CONSOLE: TicketMutex<Console> = TicketMutex::new(Console::new());

/// Represents a single character on the framebuffer console
#[derive(Copy, Clone)]
struct ConsoleChar {
    character: char,
    color: u32,
}

/// Buffer of characters in the console
struct ConsoleBuffer {
    chars: [[ConsoleChar; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
}

/// Framebuffer console
pub struct Console {
    buffer: ConsoleBuffer,
    cursor_x: usize,
    cursor_y: usize,
}

#[allow(unused)]
impl Console {
    /// Create a new console
    pub const fn new() -> Self {
        Self {
            buffer: ConsoleBuffer {
                chars: [[ConsoleChar {
                    character: '\0',
                    color: 0,
                }; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
            },
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Write a char to the console
    pub fn write_char(&mut self, character: char, color: u32) {
        // Write the character to the buffer
        match character {
            // Newline
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            // Carriage return
            '\r' => self.cursor_x = 0,
            // Tab
            '\t' => {
                for _ in 0..4 {
                    self.write_char(' ', color);
                }
            }
            // Backspace
            '\x08' => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.write_char(' ', color);
                    self.cursor_x -= 1;
                }
            }
            // Any other character
            _ => {
                self.buffer.chars[self.cursor_y][self.cursor_x] = ConsoleChar { character, color };
                self.cursor_x += 1;
            }
        }

        // If we've reached the end of the line, move to the next line
        if self.cursor_x >= CONSOLE_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // If we've reached the end of the console, scroll
        if self.cursor_y >= CONSOLE_HEIGHT {
            self.scroll();
        }
    }

    /// Write a string to the console
    pub fn write_str(&mut self, string: &str, color: u32) {
        for character in string.chars() {
            self.write_char(character, color);
        }
        // Flush the console to the framebuffer
        self.flush();
    }

    /// Clear the console
    pub fn clear(&mut self) {
        for y in 0..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                self.buffer.chars[y][x] = ConsoleChar {
                    character: '\0',
                    color: 0,
                };
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Scroll the console
    fn scroll(&mut self) {
        // Move all lines up by one
        for y in 1..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                self.buffer.chars[y - 1][x] = self.buffer.chars[y][x];
            }
        }

        // Clear the last line
        for x in 0..CONSOLE_WIDTH {
            self.buffer.chars[CONSOLE_HEIGHT - 1][x] = ConsoleChar {
                character: '\0',
                color: 0,
            };
        }
        self.cursor_y = CONSOLE_HEIGHT - 1;
    }

    /// Flush the console to the framebuffer
    fn flush(&self) {
        for y in 0..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                // Draw the character to the framebuffer
                FRAMEBUFFER.lock().draw_char(
                    x * FONT_WIDTH + 1,  // Add a 1 pixel margin between characters
                    y * FONT_HEIGHT + 1, // Add a 1 pixel margin between lines
                    self.buffer.chars[y][x].character,
                    self.buffer.chars[y][x].color,
                );
            }
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str(string, 0xFFFFFFFF);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        CONSOLE.lock().write_fmt(format_args!($($arg)*)).unwrap();
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        CONSOLE.lock().write_fmt(format_args!($($arg)*)).unwrap();
        CONSOLE.lock().write_char('\n', 0xFFFFFFFF);
    }
}
