mod cpu;
mod exceptions;
mod gdt;
mod idt;
mod serial;

use core::fmt::Write;
use core::{
    borrow::{Borrow, BorrowMut},
    ptr::addr_of,
};

use cpu::*;

use spin::mutex::spin::SpinMutex;

use ignore_result::Ignore;

use gdt::{tss::Tss, Gdt};
use lazy_static::lazy_static;

use serial::{ComPort, SerialPort};

use idt::*;

// Define the Api struct. This struct will be used to...
pub struct Api;

// Initialize global data structures for the bootstrap processor (BSP)
lazy_static! {
    // Stack for ring 0 interrupts
    static ref BSP_RING0_INT_STACK: [u8; 4096] = [0u8; 4096];
    // Task state segment (TSS)
    static ref BSP_TSS: Tss = Tss::new(addr_of!(BSP_RING0_INT_STACK) as u64);
    // Global descriptor table (GDT)
    static ref BSP_GDT: Gdt = Gdt::new(&BSP_TSS);
    // Interrupt descriptor table (IDT)
    static ref BSP_IDT: SpinMutex<Idt> = SpinMutex::from(Idt::new());
}

// Provide the implementation of the Api trait for the Api struct
impl crate::arch::Api for Api {
    // Define the logger type
    type Logger = SerialPort;

    // Get a new logger instance
    fn get_logger() -> Self::Logger {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }
    // Halt the calling LP
    fn halt() -> ! {
        unsafe { asm_halt() }
    }
    // Kernel Panic
    fn panic() -> ! {
        unsafe { asm_halt() }
    }
    // Read a byte from the specified port
    fn inb(port: u16) -> u8 {
        unsafe { asm_inb(port) }
    }
    // Write a byte to the specified port
    fn outb(port: u16, val: u8) {
        unsafe { asm_outb(port, val) }
    }
    // Initialize the bootstrap processor (BSP)
    fn init_bsp() {
        // This routine is run by the bootsrap processor to initilize itself priot to bringing up the kernel.

        let mut logger = SerialPort::try_new(ComPort::COM1).unwrap();

        writeln!(&mut logger, "Initializing the bootstrap processor...").ignore();

        BSP_GDT.load();
        writeln!(&mut logger, "Loaded GDT").ignore();
        Gdt::reload_segment_regs();
        writeln!(&mut logger, "Reloaded segment registers").ignore();
        Gdt::load_tss();
        writeln!(&mut logger, "Loaded TSS").ignore();

        writeln!(&mut logger, "Registering exception ISRs in the IDT").ignore();
        exceptions::load_exceptions(BSP_IDT.lock().borrow_mut());
        writeln!(&mut logger, "Exception ISRs registered").ignore();

        writeln!(&mut logger, "Attempting to load IDT").ignore();
        BSP_IDT.lock().borrow().load();
        writeln!(&mut logger, "Loaded IDT").ignore();
    }
    // Initialize the application processors (APs)
    fn init_ap() {
        // This routine is run by each application processor to initialize itself prior to being handed off to the scheduler.
    }
}
