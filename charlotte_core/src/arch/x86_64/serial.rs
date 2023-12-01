use spin::mutex::{TicketMutex, TicketMutexGuard};
use core::fmt::Write;
use core::ops::Drop;
use x86_64::structures::port::{PortRead, PortWrite};

pub static COM1_IO_PORT: u16 = 0x3f8;

pub struct SerialPort {
        base_port: u16,
}
#[derive(Debug)]
pub struct Error;

impl SerialPort {
        pub fn new(base_port: u16) -> Result<Self, Error> {
                unsafe {
                        u8::write_to_port(base_port + 1, 0x00);    // Disable all interrupts
                        u8::write_to_port(base_port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
                        u8::write_to_port(base_port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
                        u8::write_to_port(base_port + 1, 0x00);    //                  (hi byte)
                        u8::write_to_port(base_port + 3, 0x03);    // 8 bits, no parity, one stop bit
                        u8::write_to_port(base_port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
                        u8::write_to_port(base_port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
                        u8::write_to_port(base_port + 4, 0x1E);    // Set in loopback mode, test the serial chip
                        u8::write_to_port(base_port + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
                
                        // Check if serial is faulty (i.e: not same byte as sent)
                        if u8::read_from_port(base_port + 0) != 0xAE {
                                Err(Error)
                        } else {
                                // If serial is not faulty set it in normal operation mode
                                // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
                                u8::write_to_port(base_port + 4, 0x0F);
                                Ok(SerialPort{base_port: base_port})
                        }
                }
        }

        unsafe fn is_transmit_empty(&self) -> u8 {
                u8::read_from_port(self.base_port + 5) & 0x20
        }
              
        fn write_serial(&self, a: u8) {
                unsafe {
                        while self.is_transmit_empty() == 0 {}
                        u8::write_to_port(self.base_port, a);
                }
        }
}

pub struct SerialWriter {
        port_me: &'static TicketMutex<SerialPort>
}

impl SerialWriter {
        pub fn new(port_me: &'static TicketMutex<SerialPort>) -> Self {
                SerialWriter { port_me: port_me }
        }
}

impl Write for SerialWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let port = self.port_me.lock();
                let mut bytes = s.bytes();
                let mut byte = Some(0u8);
                while byte != None {
                        /*Handle newlines*/
                        if byte == Some(b'\\') {
                                byte = bytes.next();
                                if byte == Some(b'n') {
                                        port.write_serial(b'\\');
                                        port.write_serial(b'r');
                                        port.write_serial(b'\\');
                                        port.write_serial(b'\n');
                                } else {
                                        port.write_serial(b'\\');
                                        match byte {
                                                Some(b) => port.write_serial(b),
                                                None => break
                                        }
                                }
                        } else {
                                match byte {
                                        Some(b) => port.write_serial(b),
                                        None => break
                                }
                        }
                        byte = bytes.next();
                }
                Ok(())
                //the TicketMutexGuard should get dropped here
        }
}

