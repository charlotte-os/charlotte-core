pub mod isa;

use core::fmt::{self, Write};

pub use isa::*;

/// A struct representing a serial port controlled by a UART
pub struct SerialPort {
    base: SerialAddr,
}

impl SerialPort {
    /// # Try to create and initialize a new SerialPort
    /// ## Arguments
    /// * `base` - The base address of the serial port
    /// ## Returns
    /// A new SerialPort if successful, otherwise None
    pub fn try_new(base: SerialAddr) -> Option<Self> {
        let mut port = match base {
            SerialAddr::Mmio(ptr) => SerialPort {
                base: SerialAddr::Mmio(ptr),
            },
            SerialAddr::IoPort(port) => SerialPort {
                base: SerialAddr::IoPort(port),
            },
            _ => return None,
        };
        (port.base + 1).write(0x00); // Disable all interrupts
        (port.base + 3).write(0x80); // Enable DLAB (set baud rate divisor)
        (port.base + 0).write(0x03); // Set divisor to 3 (lo byte) 38400 baud
        (port.base + 1).write(0x00); // (hi byte)
        (port.base + 3).write(0x03); // 8 bits, no parity, one stop bit
        (port.base + 2).write(0xC7); // Enable FIFO, clear them, with 14-byte threshold
        (port.base + 4).write(0x0B); // IRQs enabled, RTS/DSR set
        (port.base + 4).write(0x1E); // Set in loopback mode, test the serial chip
        (port.base + 0).write(0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if port.base.read() != 0xAE {
            None
        } else {
            (port.base + 4).write(0x0F);
            Some(port)
        }
    }
    /// # Check if the transmit buffer is empty
    /// ## Returns
    /// Non-zero if the buffer is empty, zero if it is not
    fn is_transmit_empty(&self) -> i32 {
        ((self.base + 5).read() & 0x20).into()
    }
    /// # Check if data has been received
    /// ## Returns
    /// True if data has been received, false if it has not
    /// This is the same as checking if the data-ready bit is set
    /// in the line status register
    fn received(&self) -> bool {
        ((self.base + 5).read() & 1) != 0
    }
}

impl Write for SerialPort {
    /// # Write a string slice to the serial port
    /// ## Arguments
    /// * `&mut self` - The serial port to write to
    /// * `s` - The string slice to write
    /// ## Returns
    /// Ok(()) if successful, Err(fmt::Error) if not
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?
        }
        Ok(())
    }
    /// # Write a character to the serial port
    /// ## Arguments
    /// * `&mut self` - The serial port to write to
    /// * `c` - The character to write
    /// ## Returns
    /// Ok(()) if successful, Err(fmt::Error) if not
    fn write_char(&mut self, c: char) -> fmt::Result {
        while self.is_transmit_empty() == 0 {}
        if c.is_ascii() {
            if c == '\n' {
                self.base.write('\r' as u8);
                self.base.write('\n' as u8);
            } else {
                self.base.write(c as u8);
            }
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}
