use core::arch::global_asm;
use core::fmt::Write;

use crate::arch::x86_64::cpu::{asm_are_interrupts_enabled, irq_disable, irq_restore};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::interrupts::apic::Apic;
use crate::arch::x86_64::interrupts::vectors::IV_HANDLERS;
use crate::logln;

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

pub fn load_handlers(idt: &mut Idt) {
    logln!("load handlers");
    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    for (gate, h) in IV_HANDLERS.iter().enumerate() {
        idt.set_gate(gate, *h, 1 << 3, true, true);
    }
    idt.load();
    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
    logln!("loaded handlers");
}

#[no_mangle]
pub extern "C" fn isr_handler(vector: u64) {
    Apic::signal_eoi();
}

global_asm! {
    include_str!("isa_handler.asm"),
}

global_asm! {
    include_str!("vectors.asm"),
}
