//! # Arch
//! This module provides a common interface for interacting with code that is specific to each supported
//! instruction set architecture (ISA). It provides a set of traits and types that can be used to interact
//! with ISA specific code in a consistent and platform independent manner.

use core::fmt::{Result, Write};

use spin::{lazy::Lazy, mutex::TicketMutex};

use crate::framebuffer::console::CONSOLE;
use crate::memory::address::UAddr;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub static LOGGER: Lazy<TicketMutex<Logger>> = Lazy::new(|| {
    TicketMutex::new(Logger {
        logger: <ArchApi as Api>::get_logger(),
    })
});

#[derive(Debug, Copy, Clone)]
pub struct PagingParams {
    pub page_size: UAddr,
    pub page_shift: UAddr,
    pub page_mask: UAddr,
}

#[derive(Debug, Copy, Clone)]
pub struct IsaParams {
    pub paging: PagingParams,
}

pub trait Api {
    type Api: Api;
    type DebugLogger: Write;
    type Serial: Serial;

    /// Each ISA implementation does something specific within this function,
    /// you should check the relevant implementation under each ISA folder linked bellow:
    /// * [X86_64](x86_64::Api::isa_init)
    fn isa_init() -> Self;

    fn get_logger() -> Self::DebugLogger;
    fn get_serial(&self) -> Self::Serial;
    fn get_paddr_width() -> u8;
    fn get_vaddr_width() -> u8;
    #[allow(unused)]
    fn halt() -> !;
    fn panic() -> !;
    fn inb(port: u16) -> u8;
    fn outb(port: u16, val: u8);
    #[allow(unused)]
    fn init_ap(&mut self);
    #[allow(unused)]
    fn start_isa_timers(&self);
    fn init_interrupts(&mut self);
    fn interrupts_enabled(&self) -> bool;
    #[allow(unused)]
    fn disable_interrupts(&mut self);
    #[allow(unused)]
    fn restore_interrupts(&mut self);
    #[allow(unused)]
    fn set_interrupt_handler(&mut self, h: fn(vector: u64), vector: u32);
    #[allow(unused)]
    fn end_of_interrupt();
}

pub trait Serial {
    fn read_char(&mut self) -> char;
    fn put_char(&mut self, c: char);
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

#[cfg(target_arch = "x86_64")]
pub const ISA_PARAMS: IsaParams = x86_64::X86_ISA_PARAMS;
#[cfg(target_arch = "aarch64")]
pub static MEMORY_PARAMS: PagingParams = aarch64::ISA_MEMORY_PARAMS;
#[cfg(target_arch = "riscv64")]
pub static MEMORY_PARAMS: PagingParams = riscv64::ISA_MEMORY_PARAMS;
