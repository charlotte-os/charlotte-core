//! # Arch
//! This module provides a common interface for interacting with code that is specific to each supported
//! instruction set architecture (ISA). It provides a set of traits and types that can be used to interact
//! with ISA specific code in a consistent and platform independent manner.

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

use core::fmt::{Result, Write};

use spin::{lazy::Lazy, mutex::TicketMutex};

use crate::{acpi::AcpiTables, framebuffer::console::CONSOLE};

pub static LOGGER: Lazy<TicketMutex<Logger>> = Lazy::new(|| {
    TicketMutex::new(Logger {
        logger: <ArchApi as Api>::get_logger(),
    })
});

pub trait Api {
    type Api: Api;
    type DebugLogger: Write;

    fn new_arch_api() -> Self;

    fn get_logger() -> Self::DebugLogger;
    fn get_paddr_width() -> u8;
    fn get_vaddr_width() -> u8;
    fn halt() -> !;
    fn panic() -> !;
    fn inb(port: u16) -> u8;
    fn outb(port: u16, val: u8);
    fn init_bsp();
    #[allow(unused)]
    fn init_ap();
    fn init_timers(&self);
    fn init_interrupts(&self);
    fn interrupts_enabled(&self) -> bool;
    fn disable_interrupts(&mut self);
    fn restore_interrupts(&mut self);
    fn end_of_interrupt(&self);

    /// Sets the acpi tables that can be used by the impl to find needed information
    fn init_acpi_tables(&mut self, tbls: &AcpiTables);
}

/// A logger that writes to both the framebuffer console and the serial port.
pub struct Logger {
    logger: <ArchApi as Api>::DebugLogger,
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> Result {
        write!(self.logger, "{}", s).unwrap();
        write!(CONSOLE.lock(), "{}", s).unwrap();
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::arch::LOGGER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    };
}

#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => {
        $crate::arch::LOGGER.lock().write_fmt(format_args!($($arg)*)).unwrap();
        $crate::arch::LOGGER.lock().write_str("\n").unwrap();
    };
}

#[cfg(target_arch = "x86_64")]
pub type ArchApi = x86_64::Api;
#[cfg(target_arch = "aarch64")]
pub type ArchApi = aarch64::Api;
#[cfg(target_arch = "riscv64")]
pub type ArchApi = riscv64::Api;
