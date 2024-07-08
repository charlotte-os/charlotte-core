use crate::arch::x86_64::cpu::{irq_disable, asm_irq_enable, asm_outb, asm_are_interrupts_enabled, irq_restore};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::serial::ComPort::COM1;
use crate::arch::x86_64::serial::SerialPort;
use crate::logln;
use crate::memory::address::VirtualAddress;
use core::arch::global_asm;
use core::fmt::Write;
use ignore_result::Ignore;

#[derive(Debug)]
#[repr(C, packed)]
struct IntStackFrame {
    pc: VirtualAddress,
    seg_sel: u16,
    flags: u64,
    stack_ptr: VirtualAddress,
    stk_seg: u16,
}

extern "C" {
    pub(crate) fn asm_iretq();
    fn isr_wrapper();
    pub(crate) fn isr_dummy();
    pub(crate) fn isr_spurious();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IntIdx {
    Timer = 0x20,
}


pub fn load_dummy_handlers(idt: &mut Idt) {
    for gate in 32..256 {
        idt.set_gate(gate, isr_dummy, 1 << 3, true, true);
    }
    idt.load();
}

pub fn set_isr(idt: &mut Idt, gate: usize, isr_ptr: unsafe extern "C" fn()) {
    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    idt.set_gate(gate, isr_ptr, 1 << 3, true, true);
    idt.load();
    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
}

#[no_mangle]
pub extern "C" fn timer_handler() {
    let mut logger = SerialPort::try_new(COM1).unwrap();

    writeln!(&mut logger, ".").ignore();
}

global_asm! {
    include_str!("isa_handler.asm"),
}
