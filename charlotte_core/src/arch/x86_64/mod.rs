//! # x86_64 Architecture Module
//! This module implements the Arch interface for the x86_64 instruction set architecture (ISA).

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
use crate::arch::x86_64::interrupts::apic::{Apic, TimerMode};
use crate::arch::{IsaParams, PagingParams};
use crate::arch::x86_64::interrupts::isa_handler::TIMER_CALLED_TIMES;
use crate::framebuffer::colors::Color;
use crate::framebuffer::framebuffer::FRAMEBUFFER;
use crate::logln;
use crate::memory::pmm::PHYSICAL_FRAME_ALLOCATOR;

mod cpu;
mod exceptions;
mod gdt;
mod global;
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

pub const X86_ISA_PARAMS: IsaParams = IsaParams {
    paging: PagingParams {
        page_size: 0x1000,
        page_shift: 0xC,
        page_mask: !0xfff,
    },
};

/// Provide the implementation of the Api trait for the Api struct
impl crate::arch::Api for Api {
    type Api = Api;
    /// Define the logger type
    type DebugLogger = SerialPort;
    type Serial = SerialPort;

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
        logln!("Enable interrupts");
        api.init_interrupts();
        api.bsp_apic.enable(BSP_IDT.lock().borrow_mut());
        api.bsp_apic.setup_timer(TimerMode::Periodic, 100000, 0);
        logln!("bus speed: {}", api.bsp_apic.tps/10000);
        logln!("============================================================\n");

        logln!("Memory self test");
        Self::pmm_self_test();
        logln!("============================================================\n");

        logln!("All x86_64 sanity checks passed, kernel main has control now");
        logln!("============================================================\n");

        api
    }

    /// Get a new logger instance
    fn get_logger() -> Self::DebugLogger {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }

    fn get_serial(&self) -> Self::Serial {
        SerialPort::try_new(ComPort::COM1).unwrap()
    }

    /// Get the number of significant physical address bits supported by the current CPU
    fn get_paddr_width() -> u8 {
        *PADDR_SIG_BITS
    }
    /// Get the number of significant virtual address bits supported by the current CPU
    fn get_vaddr_width() -> u8 {
        *VADDR_SIG_BITS
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
        asm_inb(port)
    }

    /// Write a byte to the specified port
    fn outb(port: u16, val: u8) {
        asm_outb(port, val)
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

    fn pmm_self_test() {
        logln!(
            "Number of Significant Physical Address Bits Supported: {}",
            Api::get_paddr_width()
        );
        logln!(
            "Number of Significant Virtual Address Bits Supported: {}",
            Api::get_vaddr_width()
        );

        logln!("Testing Physical Memory Manager");
        logln!("Performing single frame allocation and deallocation test.");
        let alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
        let alloc2 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
        match alloc {
            Ok(frame) => {
                logln!("Allocated frame with physical base address: {:?}", frame);
                let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
                logln!("Deallocated frame with physical base address: {:?}", frame);
            }
            Err(e) => {
                logln!("Failed to allocate frame: {:?}", e);
            }
        }
        let alloc3 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
        logln!("alloc2: {:?}, alloc3: {:?}", alloc2, alloc3);
        let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc2.unwrap());
        let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc3.unwrap());
        logln!("Single frame allocation and deallocation test complete.");
        logln!("Performing contiguous frame allocation and deallocation test.");
        let contiguous_alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(256, 64);
        match contiguous_alloc {
            Ok(frame) => {
                logln!(
                    "Allocated physically contiguous region with physical base address: {:?}",
                    frame
                );
                let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
                logln!(
                    "Deallocated physically contiguous region with physical base address: {:?}",
                    frame
                );
            }
            Err(e) => {
                logln!("Failed to allocate contiguous frames: {:?}", e);
            }
        }
        logln!("Contiguous frame allocation and deallocation test complete.");
        logln!("Physical Memory Manager test suite finished.");
    }
}
