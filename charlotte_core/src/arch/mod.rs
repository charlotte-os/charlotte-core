//! # Instruction Set Architecture Abstraction Subsystem
//! This module provides a common interface for interacting with code that is specific to each supported
//! instruction set architecture (ISA). It provides a set of traits and types that can be used to interact
//! with ISA specific code in a consistent and platform independent manner.

use core::fmt::{Result, Write};

use spin::{lazy::Lazy, mutex::TicketMutex};

use crate::framebuffer::console::CONSOLE;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

use core::fmt::*;
use core::result::Result;

use spin::{lazy::Lazy, mutex::TicketMutex};

use crate::{acpi::AcpiTables, framebuffer::console::CONSOLE};

use crate::memory::address::{PhysicalAddress, VirtualAddress};

pub static LOGGER: Lazy<TicketMutex<Logger>> = Lazy::new(|| {
    TicketMutex::new(Logger {
        logger: <ArchApi as Api>::get_logger(),
    })
});

/// Common interface for ISA specific memory map structures.
pub trait MemoryMap: Clone + Drop {
    type Error;
    type Flags;

    /// Loads the page map into the logical processor.
    unsafe fn load(&self);

    /// Maps a page at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to map the page to
    /// * `paddr` - The physical base address of the page frame to be mapped
    /// * `flags` - The flags to apply to the page table entry
    fn map_page(
        &mut self,
        vaddr: VirtualAddress,
        paddr: PhysicalAddress,
        flags: Self::Flags,
    ) -> Result<(), Self::Error>;

    /// Unmaps a page from the given page map at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to unmap.
    /// # Returns
    /// Returns an error of type `Self::Error` if unmapping fails or the physical address that was
    /// previously mapped to the given virtual address if successful.
    fn unmap_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error>;

    /// Maps a large page (2 MiB) at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to map.
    /// * `paddr` - The physical address to map.
    /// * `flags` - The flags to apply to the page table entry.
    /// # Returns
    /// Returns an error of type `Self::Error` if mapping fails.
    fn map_large_page(
        &mut self,
        vaddr: VirtualAddress,
        paddr: PhysicalAddress,
        flags: Self::Flags,
    ) -> Result<(), Self::Error>;

    /// Unmaps a large page from the given page map at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to unmap.
    /// # Returns
    /// Returns an error of type `Self::Error` if unmapping fails or the physical address that was
    /// previously mapped to the given virtual address if successful.
    fn unmap_large_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error>;

    /// Maps a huge page (1 GiB) at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to map.
    /// * `paddr` - The physical address to map.
    /// * `flags` - The flags to apply to the page table entry.
    /// # Returns
    /// Returns an error of type `Self::Error` if mapping fails.
    fn map_huge_page(
        &mut self,
        vaddr: VirtualAddress,
        paddr: PhysicalAddress,
        flags: Self::Flags,
    ) -> Result<(), Self::Error>;

    /// Unmaps a huge page from the given page map at the given virtual address.
    /// # Arguments
    /// * `vaddr` - The virtual address to unmap.
    /// # Returns
    /// Returns an error of type `Self::Error` if unmapping fails or the physical address that was
    /// previously mapped to the given virtual address if successful.
    fn unmap_huge_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error>;
}

/// Common interface for ISA specific functionality.
pub trait Api {
    type Api: Api;
    type DebugLogger: Write;
    type MemMap: MemoryMap;

    /// Each ISA implementation does something specific within this function,
    /// you should check the relevant implementation under each ISA folder linked bellow:
    /// * [X86_64](x86_64::Api::isa_init)
    fn isa_init() -> Self;

    /// logging
    fn get_logger() -> Self::DebugLogger;
    /// memory
    fn get_paddr_width() -> u8;
    fn get_vaddr_width() -> u8;
    fn validate_paddr(raw: usize) -> bool;
    fn validate_vaddr(raw: u64) -> bool;
    /// cpu
    fn halt() -> !;
    fn panic() -> !;
    /// io ports
    fn inb(port: u16) -> u8;
    fn outb(port: u16, val: u8);
    /// processor initialization
    fn init_bsp();
    #[allow(unused)]
    fn init_timers(&self);
    fn init_interrupts(&mut self);
    fn interrupts_enabled(&self) -> bool;
    #[allow(unused)]
    fn disable_interrupts(&mut self);
    #[allow(unused)]
    fn restore_interrupts(&mut self);
    #[allow(unused)]
    fn end_of_interrupt(&self);
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
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
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
