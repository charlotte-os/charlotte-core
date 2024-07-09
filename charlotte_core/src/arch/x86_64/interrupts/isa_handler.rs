use crate::arch::x86_64::cpu::{asm_are_interrupts_enabled, irq_disable, irq_restore};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::interrupts::apic::{Apic, IV_HANDLER_FN};
use crate::arch::x86_64::interrupts::vectors::IV_HANDLERS;
use core::arch::global_asm;

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
    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    for (gate, h) in IV_HANDLERS.iter().enumerate() {
        let i = gate + 32;
        if i == 39 {
            idt.set_gate(i, isr_dummy, 1 << 3, false, true);
            continue;
        }
        idt.set_gate(i, *h, 1 << 3, false, true);
    }
    idt.load();
    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
}

#[no_mangle]
pub extern "C" fn isr_handler(vector: u64) {
    let called_by = vector - 32;
    unsafe {
        if let Some(h) = IV_HANDLER_FN[called_by as usize] {
            h()
        } else {
            Apic::signal_eoi();
        }
    }
}

global_asm! {
    include_str!("isa_handler.asm"),
}

global_asm! {
    include_str!("vectors.asm"),
}
