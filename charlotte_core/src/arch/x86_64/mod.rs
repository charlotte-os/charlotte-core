mod cpu;
mod exceptions;
mod gdt;
mod idt;
mod serial;

use core::{ptr::addr_of, borrow::{BorrowMut, Borrow}};
use core::fmt::Write;

use cpu::*;

use spin::mutex::spin::SpinMutex;

use ignore_result::Ignore;

use gdt::{Gdt, tss::Tss};
use lazy_static::lazy_static;

use serial::{SerialPort, ComPort};



use idt::*;

pub struct Api;


lazy_static! {
        static ref BSP_RING0_INT_STACK: [u8; 4096] = [0u8; 4096];
        static ref BSP_TSS: Tss = Tss::new(addr_of!(BSP_RING0_INT_STACK) as u64);
        static ref BSP_GDT: Gdt = Gdt::new(&BSP_TSS);
        static ref BSP_IDT: SpinMutex<Idt> = SpinMutex::from(Idt::new());
}

impl crate::arch::Api for Api {
        type Logger = SerialPort;

        fn get_logger() -> Self::Logger {
                SerialPort::try_new(ComPort::COM1).unwrap()
        }
        fn halt() -> ! {
                unsafe {asm_halt()}
        }
        fn panic() -> ! {
                unsafe {asm_halt()}
        }
        fn inb(port: u16) -> u8 {
                unsafe {asm_inb(port)}
        }
        fn outb(port: u16, val: u8) {
                unsafe {asm_outb(port, val)}
        }
        fn init_bsp() {
                /*This routine is run by the bootsrap processor to initilize itself priot to bringing up the kernel.*/

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
        fn init_ap() {
                /*This routine run by each application processor to initialize itself prior to  being handed off to the scheduler.*/
        }
}