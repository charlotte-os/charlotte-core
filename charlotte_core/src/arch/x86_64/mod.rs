mod serial;

use core::arch::{asm, global_asm};
use lazy_static::lazy_static;
use spin::mutex::TicketMutex;
use x86_64::structures::port::{PortRead, PortWrite};

use self::serial::*;

lazy_static! {
        static ref COM1: TicketMutex<SerialPort> = TicketMutex::from(SerialPort::new(serial::COM1_IO_PORT).unwrap());
}

pub struct ArchApi;

impl super::Arch for ArchApi {
        type Logger = SerialWriter;

        fn halt() -> ! {
                unsafe {
                        asm!("cli; hlt");
                }
                loop {}
        }
        fn get_logger() -> Self::Logger {
                SerialWriter::new(&COM1)
        }
}

