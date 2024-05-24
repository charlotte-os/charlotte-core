//! # x86_64 Architecture Module
//! This module implements the Arch interface for the x86_64 instruction set architecture (ISA).

mod cpu;
mod exceptions;
mod gdt;
mod idt;
mod interrupts;
mod memory;
mod serial;
mod timers;

use core::fmt::Write;
use core::str;
use core::{
    borrow::{Borrow, BorrowMut},
    ptr::addr_of,
};

use spin::lazy::Lazy;
use spin::mutex::spin::SpinMutex;

use cpu::*;
use gdt::{tss::Tss, Gdt};
use idt::*;
use serial::{ComPort, SerialPort};

use crate::acpi::{parse, AcpiInfo};
use crate::arch::x86_64::interrupts::apic::Apic;
use crate::framebuffer::colors::Color;
use crate::framebuffer::framebuffer::FRAMEBUFFER;
use crate::logln;
use crate::memory::pmm::PHYSICAL_FRAME_ALLOCATOR;

mod cpu;
mod exceptions;
mod gdt;
mod idt;
mod interrupts;
mod serial;
mod timers;

/// The Api struct is used to provide an implementation of the ArchApi trait for the x86_64 architecture.
pub struct Api {
    acpi_info: AcpiInfo,
    bsp_apic: Apic,
    irq_flags: u64,
}

static BSP_RING0_INT_STACK: [u8; 4096] = [0u8; 4096];
static BSP_TSS: Lazy<Tss> = Lazy::new(|| Tss::new(addr_of!(BSP_RING0_INT_STACK) as u64));
static BSP_GDT: Lazy<Gdt> = Lazy::new(|| Gdt::new(&BSP_TSS));
static BSP_IDT: SpinMutex<Idt> = SpinMutex::new(Idt::new());

/// Provide the implementation of the Api trait for the Api struct
impl crate::arch::Api for Api {
    type Api = Api;
    /// Define the logger type
    type DebugLogger = SerialPort;
    type MemMap = memory::PageMap;

    fn isa_init() -> Self {
        FRAMEBUFFER.lock().clear_screen(Color::BLACK);

        logln!("Initializing the bootstrap processor");
        Api::init_bsp();
        logln!("============================================================\n");
        logln!("Parsing ACPI information");
        let tbls = parse();
        logln!("============================================================\n");
        let mut api = Api {
            acpi_info: tbls,
            bsp_apic: Apic::new(tbls.madt()),
            irq_flags: 0,
        };
        logln!("============================================================\n");
        logln!("Enable the interrupts");
        api.init_interrupts();
        logln!("============================================================\n");

        logln!("Memory self test");
        Self::pmm_self_test();
        logln!("============================================================\n");

        logln!("All x86_64 sanity checks passed, kernel main has control now");
        logln!("============================================================\n");

        api
    }

    /// Get a new logger instance
    #[inline]
    fn get_logger() -> Self::DebugLogger {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }

    fn get_serial(&self) -> Self::Serial {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }

    /// Get the number of significant physical address bits supported by the current CPU
    #[inline]
    fn get_paddr_width() -> u8 {
        *memory::PADDR_SIGBITS
    }
    /// Get the number of significant virtual address bits supported by the current CPU
    #[inline]
    fn get_vaddr_width() -> u8 {
        *memory::VADDR_SIGBITS
    }
    /// Validates a physical address in accordance with the x86_64 architecture
    fn validate_paddr(raw: usize) -> bool {
        // Non-significant bits must be zero
        let unused_bitmask = !(1 << Self::get_paddr_width() - 1);
        (raw & unused_bitmask) == 0
    }
    /// Validates a virtual address in accordance with the x86_64 architecture
    fn validate_vaddr(raw: u64) -> bool {
        // Canonical form check
        let unused_bitmask = 1 << Self::get_vaddr_width() - 1;
        let msb = (raw & (1 << (Self::get_vaddr_width() - 1))) > 0;
        match msb {
            false => raw & unused_bitmask == 0,
            true => raw & unused_bitmask == unused_bitmask
        }
    }
    /// Halt the calling LP
    #[inline]
    fn halt() -> ! {
        unsafe { asm_halt() }
    }

    /// Kernel Panic
    #[inline]
    fn panic() -> ! {
        unsafe { asm_halt() }
    }

    /// Read a byte from the specified port
    #[inline]
    fn inb(port: u16) -> u8 {
        unsafe { asm_inb(port) }
    }

    /// Write a byte to the specified port
    #[inline]
    fn outb(port: u16, val: u8) {
        unsafe { asm_outb(port, val) }
    }

    /// Initialize the bootstrap processor (BSP)
    ///
    ///  Initialize the application processors (APs)
    fn init_ap(&mut self) {
        //! This routine is run by each application processor to initialize itself prior to being handed off to the scheduler.
    }

    fn init_timers(&self) {
        unimplemented!()
    }

    fn init_interrupts(&mut self) {
        self.bsp_apic.init()
    }

    fn interrupts_enabled(&self) -> bool {
        asm_are_interrupts_enabled()
    }

    fn disable_interrupts(&mut self) {
        self.irq_flags = asm_irq_disable();
    }

    fn restore_interrupts(&mut self) {
        asm_irq_restore(self.irq_flags);
    }

    fn end_of_interrupt(&self) {}
}

impl Api {
    /// Get the number of significant physical address bits supported by the current CPU
    fn get_paddr_width() -> u8 {
        *PADDR_SIG_BITS
    }
    /// Get the number of significant virtual address bits supported by the current CPU
    fn get_vaddr_width() -> u8 {
        *VADDR_SIG_BITS
    }

    fn init_bsp() {
        //! This routine is run by the bootstrap processor to initialize itself prior to bringing up the kernel.
        logln!("Processor information:");
        BSP_GDT.load();
        logln!("Loaded GDT");
        Gdt::reload_segment_regs();
        logln!("Reloaded segment registers");
        Gdt::load_tss();
        logln!("Loaded TSS");

        logln!("Registering exception ISRs in the IDT");
        exceptions::load_exceptions(BSP_IDT.lock().borrow_mut());
        logln!("Exception ISRs registered");

        logln!("Attempting to load IDT");
        BSP_IDT.lock().borrow().load();
        logln!("Loaded IDT");

        let mut vendor_string = [0u8; 12];
        unsafe { asm_get_vendor_string(&mut vendor_string) }
        logln!("CPU Vendor ID: {}", str::from_utf8(&vendor_string).unwrap());
    }
    /// Initialize the application processors (APs)
    /// This routine is run by each application processor to initialize itself prior to being handed off to the scheduler.
    fn init_ap() {
        todo!("Implement init_ap() for the x86_64 architecture once a scheduler exists.");
    }

    fn init_timers(&self) {
        unimplemented!()
    }

    fn init_interrupts(&self) {
        if let Some(tables) = self.tables {
            logln!("Initializing interrupt controllers");
            if !check_apic_is_present() {
                panic!("APIC is not present according to CPUID");
            }
            // enable the lapic
            logln!("Enable LAPIC");
            set_apic_base(get_apic_base());
            logln!(
                "Spurious Interrupt vector register {:X}",
                read_apic_reg(tables.madt(), 0xF0)
            );
            write_apic_reg(
                tables.madt(),
                0xF0,
                read_apic_reg(tables.madt(), 0xF0) | (1 << 8),
            );
        } else {
            panic!("Interrupts initialization without initializing ACPI tables");
        }
    }

    fn init_acpi_tables(&mut self, tbls: &AcpiTables) {
        // Copy the tables passed in to the API
        self.tables = Some(tbls.clone());
    }
}
