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

use crate::acpi::madt::{IoApic, MadtEntry};
use crate::acpi::AcpiTables;
use crate::arch::x86_64::interrupts::apic::{
    check_apic_is_present, get_apic_base, read_apic_reg, set_apic_base, write_apic_reg,
};
use crate::logln;

use self::interrupts::apic_consts::APIC_REG_EOI;

/// The Api struct is used to provide an implementation of the ArchApi trait for the x86_64 architecture.
pub struct Api {
    pub tables: Option<AcpiTables>,
    io_apics: [Option<IoApic>; 64],
    #[allow(dead_code)]
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

    fn new_arch_api() -> Self {
        Self {
            tables: None,
            io_apics: [None; 64],
            irq_flags: 0,
        }
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
        //! This routine is run by the bootstrap processor to initialize itself prior to bringing up the kernel.

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
            if !check_apic_is_present() {
                panic!("APIC is not present according to CPUID");
            }
            // enable the lapic
            set_apic_base(get_apic_base());
            write_apic_reg(
                tables.madt(),
                0xF0,
                read_apic_reg(tables.madt(), 0xF0) | (1 << 8),
            );
            // enable irqs
            asm_irq_enable();
        } else {
            panic!("Interrupts initialization without initializing ACPI tables");
        }
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

    fn register_interrupt_dispatcher(&mut self) {}

    fn end_of_interrupt(&self) {
        if let Some(tables) = self.tables {
            if self.interrupts_enabled() {
                write_apic_reg(tables.madt(), APIC_REG_EOI, 0);
            } else {
                logln!("Attempt to signal end of interrupt with interrupts disabled");
            }
        } else {
            panic!("Tried to signal end of interrupt without initializing ACPI tables");
        }
    }

    fn init_acpi_tables(&mut self, tbls: &AcpiTables) {
        // Copy the tables passed in to the API
        self.tables = Some(tbls.clone());
        if let Some(tables) = self.tables {
            let mut idx = 0;
            let mut madt = tables.madt().iter();
            while let Some(madt_entry) = madt.next() {
                match madt_entry {
                    MadtEntry::IOApic(entry) => {
                        self.io_apics[idx] = Some(entry);
                        idx += 1;
                    }
                    _ => continue,
                }
            }
        }
        logln!("{:?}", self.io_apics);
    }
}
