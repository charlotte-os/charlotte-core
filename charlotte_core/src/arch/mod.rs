//! # Arch
//! This module provides a common interface for interacting with code that is specific to each supported
//! instruction set architecture (ISA). It provides a set of traits and types that can be used to interact
//! with ISA specific code in a consistent and platform independent manner.

use core::fmt;
use core::fmt::Write;
use core::result::Result;

use spin::{lazy::Lazy, mutex::TicketMutex};

use crate::framebuffer::console::CONSOLE;
use crate::memory::address::{PhysicalAddress, UAddr, VirtualAddress};

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

pub trait MemoryMap {
    type Error;
    type Flags;

    /// Loads the page map into the logical processor.
    unsafe fn load(&self) -> Result<(), Self::Error>;

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
pub enum HwTimerMode {
    OneShot,
    Recurrent,
}

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
    type MemoryMap: MemoryMap;
    type Serial: Serial;

    /// Each ISA implementation does something specific within this function,
    /// you should check the relevant implementation under each ISA folder linked bellow:
    /// * [X86_64](x86_64::Api::isa_init)
    fn isa_init() -> Self;

    fn get_logger() -> Self::DebugLogger;
    fn get_serial(&self) -> Self::Serial;
    fn get_paddr_width() -> u8;
    fn get_vaddr_width() -> u8;
    fn validate_paddr(raw: usize) -> bool;
    fn validate_vaddr(raw: u64) -> bool;
    #[allow(unused)]
    fn halt() -> !;
    fn panic() -> !;
    fn inb(port: u16) -> u8;
    fn outb(port: u16, val: u8);
    #[allow(unused)]
    fn init_ap(&mut self);
    /// Sets up the ISA specific timer(s)
    /// ## Notes:
    /// * for ISAs with only one timer timer_id is ignored
    /// * some ISAs have timers that can't be as precise as say 10 tps, check the ISA manuals for details
    fn setup_isa_timer(&mut self, tps: u32, mode: HwTimerMode, timer_id: u16);
    fn start_isa_timers(&self);
    fn pause_isa_timers(&self);
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
    fn write_str(&mut self, s: &str) -> fmt::Result {
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
