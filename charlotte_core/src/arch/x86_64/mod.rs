//! # x86_64 Architecture Module
//! This module implements the Arch interface for the x86_64 instruction set architecture (ISA).

mod cpu;
mod exceptions;
mod gdt;
mod idt;
mod interrupts;
mod serial;
mod timers;

use core::fmt::Write;
use core::{
    borrow::{Borrow, BorrowMut},
    ptr::addr_of,
};

use core::str;

use cpu::*;

use spin::lazy::Lazy;
use spin::mutex::spin::SpinMutex;

use gdt::{tss::Tss, Gdt};

use serial::{ComPort, SerialPort};

use idt::*;

use crate::acpi::AcpiTables;
use crate::arch::x86_64::timers::lapic::{check_apic_is_present, list_apics};
use crate::logln;

/// The Api struct is used to provide an implementation of the ArchApi trait for the x86_64 architecture.
pub struct Api {
    pub tables: Option<AcpiTables>,
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

    fn new_arch_api() -> Self {
        Self { tables: None }
    }

    /// Get a new logger instance
    fn get_logger() -> Self::DebugLogger {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }
    /// Get the number of significant physical address bits supported by the current CPU
    fn get_paddr_width() -> u8 {
        *cpu::PADDR_SIG_BITS
    }
    /// Get the number of significant virtual address bits supported by the current CPU
    fn get_vaddr_width() -> u8 {
        *cpu::VADDR_SIG_BITS
    }
    /// Halt the calling LP
    fn halt() -> ! {
        unsafe { asm_halt() }
    }
    /// Kernel Panic
    fn panic() -> ! {
        unsafe { asm_halt() }
    }
    /// Read a byte from the specified port
    fn inb(port: u16) -> u8 {
        unsafe { asm_inb(port) }
    }
    /// Write a byte to the specified port
    fn outb(port: u16, val: u8) {
        unsafe { asm_outb(port, val) }
    }
    /// Initialize the bootstrap processor (BSP)
    fn init_bsp() {
        //! This routine is run by the bootsrap processor to initilize itself priot to bringing up the kernel.

        logln!("Initializing the bootstrap processor...");

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
        unsafe { cpu::asm_get_vendor_string(&mut vendor_string) }
        logln!("CPU Vendor ID: {}", str::from_utf8(&vendor_string).unwrap());
    }
    ///
    ///  Initialize the application processors (APs)
    fn init_ap() {
        //! This routine is run by each application processor to initialize itself prior to being handed off to the scheduler.
    }

    fn init_timers(&self) {
        unimplemented!()
    }

    fn init_interrupts(&self) {
        if let Some(tables) = self.tables {
            logln!("Initializing interrupt controllers");
            let apic_presence = check_apic_is_present();
            logln!("apic_presence {}", apic_presence);
            let lapic_list = list_apics(tables.madt());
            logln!("LapicList {:?}", lapic_list);
        } else {
            panic!("Interrupts intialization without initializing ACPI tables");
        }
    }

    fn init_acpi_tables(&mut self, tbls: &AcpiTables) {
        // Copy the tables passed in to the API
        self.tables = Some(tbls.clone());
    }
}
