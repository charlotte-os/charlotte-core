use core::arch::x86_64::{__cpuid, _mm_lfence, __rdtscp, _rdtsc, _mm_pause};
use core::ptr;
use core::time::Duration;
use spin::Mutex;

use crate::acpi::madt::{Madt, MadtEntry};
use crate::arch::x86_64::cpu::{asm_irq_disable, asm_irq_restore, clear_msr_bit, set_msr_bit};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::interrupts::apic_consts::{
    APIC_DISABLE, APIC_NMI, DESTINATION_FORMAT, DIVIDE_CONFIGURATION_FOR_TIMER, LAPIC_VERSION,
    LOGICAL_DESTINATION, LVT_LINT0, LVT_LINT1, LVT_PERFORMANCE_MONITORING_COUNTERS, LVT_TIMER,
    SPURIOUS_INTERRUPT_VECTOR, TASK_PRIORITY_TPR,
};
use crate::arch::x86_64::interrupts::isa_handler::set_isr;

const FEAT_EDX_APIC: u32 = 1 << 9;
const APIC_MSR: u32 = 0x1B;
const APIC_BASE_MSR_ENABLE: u32 = 0x800;

pub struct Apic {
    base_phys_addr: usize,
    base_mapped_addr: Option<usize>,
    pub lvt_max: u8,
}

impl Apic {
    pub fn new(madt: &Madt) -> Self {
        let addr = Self::get_apic_addr(madt);

        Apic {
            base_phys_addr: addr,
            base_mapped_addr: None,
            lvt_max: 0,
        }
    }

    pub fn is_present() -> bool {
        let cpuid = unsafe { __cpuid(1) };
        (cpuid.edx & FEAT_EDX_APIC) == FEAT_EDX_APIC
    }

    fn get_addr(&self) -> usize {
        self.base_mapped_addr.unwrap_or(self.base_phys_addr)
    }

    #[allow(unused)]
    pub fn get_apic_addr(madt: &Madt) -> usize {
        let mut addr = madt.local_apic_addr() as usize;
        let mut itr = madt.iter();
        for entry in itr {
            if let MadtEntry::LocalApicAddressOverride(addr_o) = entry {
                addr = addr_o.local_apic_address as usize;
            }
        }

        addr
    }

    pub fn write_apic_reg(&self, offset: u32, value: u32) {
        let addr = (self.get_addr() + offset as usize) as *mut u32;
        unsafe { ptr::write_volatile(addr, value) }
    }

    pub fn read_apic_reg(&self, offset: u32) -> u32 {
        let addr = (self.get_addr() + offset as usize) as *const u32;
        unsafe { ptr::read_volatile(addr) }
    }

    // pub fn set_apic_base(&mut self, base: u32) {
    //     if !is_aligned(base as u64, 0x1000) {
    //         panic!("APIC base address must be page aligned to 4KB, per Intel SDM");
    //     }
    //     let prev = read_msr(APIC_MSR);
    //     // shift base 12 bits
    //     let new_value = (base << 12) | (prev.edx & 0xFFF);
    //
    //     write_msr(APIC_MSR, MSRValue { eax: eax as u32, edx: edx as u32 });
    // }

    pub fn init(&mut self) {
        let flags = asm_irq_disable();
        // If the apic is not present according to cpuid
        if !Apic::is_present() {
            panic!("APIC is not present, and is required!")
        }

        let ver_reg = self.read_apic_reg(LAPIC_VERSION);
        let max_lvt = (0xffu32 << 16) & ver_reg;
        // this is a valid downcast, by moving the max_lvt 16 bits to the right, we get the max lvt
        // in the lower 8 bits of the u32 and can then transmute it to an u8.
        self.lvt_max = ((max_lvt >> 16) + 1) as u8;

        // temporarily disable apic interrupts
        clear_msr_bit(APIC_MSR, 8);

        // initialize the APIC to known state
        let base = self.get_addr();
        if base != 0xFEE00000 {
            panic!("APIC base address is not 0xFEE00000, it is {:#X}", base);
        }

        self.write_apic_reg(DESTINATION_FORMAT, 0x0FFFFFFFF);
        let ldf = self.read_apic_reg(LOGICAL_DESTINATION) & 0x00FFFFFF;
        self.write_apic_reg(LOGICAL_DESTINATION, ldf);
        self.write_apic_reg(LVT_TIMER, APIC_DISABLE);
        self.write_apic_reg(LVT_PERFORMANCE_MONITORING_COUNTERS, APIC_NMI);
        self.write_apic_reg(LVT_LINT0, APIC_DISABLE);
        self.write_apic_reg(LVT_LINT1, APIC_DISABLE);
        self.write_apic_reg(TASK_PRIORITY_TPR, 0);

        // re-enable apic interrupts
        set_msr_bit(APIC_MSR, 8);
        asm_irq_restore(flags);
    }

    pub fn enable(&mut self, idt: &mut Idt) {
        // Map spurious interrupt to IRQ 39 which is using a dummy isr
        self.write_apic_reg(SPURIOUS_INTERRUPT_VECTOR, 0x29 + APIC_BASE_MSR_ENABLE);
        // set the timer interrupt handler
        set_isr(idt, 0x20, handle_timer_interrupt);
        // map the APIC timer to IRQ 0x20
        self.write_apic_reg(LVT_TIMER, 0x20);
        self.write_apic_reg(DIVIDE_CONFIGURATION_FOR_TIMER, 0x3);
    }

    fn measure_tsc_duration(duration: Duration) -> u64 {
        unsafe {
            _mm_lfence(); // Serialize
            let start_tsc = __rdtscp(&mut 0);
            _mm_lfence(); // Serialize

            let start_time = _rdtsc();

            // Busy-wait loop for the specified duration
            let end_time = start_time + duration.as_nanos() as u64;
            while _rdtsc() < end_time {
                _mm_pause();
            }

            _mm_lfence(); // Serialize
            let end_tsc = __rdtscp(&mut 0);
            _mm_lfence(); // Serialize

            end_tsc - start_tsc
        }
    }

    fn calculate_bus_speed(ticks: u64, duration: Duration) -> u64 {
        ticks / duration.as_secs()
    }

    fn calculate_ticks_per_second(&self) -> u64 {
        let duration = Duration::from_secs(1);
        let ticks = Self::measure_tsc_duration(duration);
        Self::calculate_bus_speed(ticks, duration)
    }

    pub fn set_timer_counter(&self, frequency: u32) {
        let ticks_per_second = self.calculate_ticks_per_second();
        let counter_value = ticks_per_second / frequency;
        self.write_apic_reg(LVT_TIMER, counter_value as u32);
    }

    pub fn set_timer_divisor(&self, divisor: u8) {
        self.write_apic_reg(DIVIDE_CONFIGURATION_FOR_TIMER, divisor as u32);
    }

    pub fn setup_timer(&self, frequency: u32, divisor: u8) {
        self.set_timer_divisor(divisor);
        self.set_timer_counter(frequency);
    }
}

extern "x86-interrupt" fn handle_timer_interrupt(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    // Handle the timer interrupt

    // Send EOI to the APIC
    unsafe {
        let eoi_addr = 0xFEE000B0 as *mut u32;
        ptr::write_volatile(eoi_addr, 0);
    }
}

// IO APIC Configuration

pub struct IoApic {
    base_phys_addr: usize,
    base_mapped_addr: Option<usize>,
    id: u8,
}

impl IoApic {
    pub fn new(base_phys_addr: usize, id: u8) -> Self {
        IoApic {
            base_phys_addr,
            base_mapped_addr: None,
            id,
        }
    }

    pub fn map_interrupt(&self, irq: u8, vector: u8) {
        // Map the IRQ to the interrupt vector in the IO APIC
    }
}

// Serial Port Interrupts

pub fn init_serial_interrupts() {
    unsafe {
        // Enable interrupts on the serial port (COM1)
        let port = 0x3F8;
        outb(port + 1, 0x01); // Enable interrupt on received data available
    }
}

extern "x86-interrupt" fn handle_serial_interrupt(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    // Read from the serial port to clear the interrupt
    let port = 0x3F8;
    unsafe {
        let _data = inb(port);
    }

    // Add serial input handling here

    // Send EOI to the APIC
    unsafe {
        let eoi_addr = 0xFEE000B0 as *mut u32;
        ptr::write_volatile(eoi_addr, 0);
    }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    value
}

// Initialize the system

pub fn initialize_system() {
    let mut idt = Idt::new();
    let madt = ...; // Initialize MADT structure here
    let apic = Apic::new(&madt);
    let io_apic_base_addr = ...; // Initialize IO APIC base address here
    let io_apic_id = ...; // Initialize IO APIC ID here
    let io_apic = IoApic::new(io_apic_base_addr, io_apic_id);

    apic.init();
    apic.enable(&mut idt);
    io_apic.map_interrupt(0, 0x20); // Map APIC timer interrupt
    io_apic.map_interrupt(4, 0x24); // Map serial interrupt

    apic.setup_timer(1000, 0x3); // Setup APIC timer with 1000Hz frequency and divisor

    idt.load();

    init_serial_interrupts();
}
