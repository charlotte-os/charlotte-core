use core::arch::global_asm;

use core::fmt::Write;
use ignore_result::Ignore;

use crate::arch::x86_64::cpu::{asm_are_interrupts_enabled, irq_disable, irq_restore};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::interrupts::apic::Apic;
use crate::arch::x86_64::interrupts::vectors::IV_HANDLERS;
use crate::arch::x86_64::serial::ComPort::COM1;
use crate::arch::x86_64::serial::SerialPort;
use crate::logln;

fn default_handler(vector: u64) {
    let mut logger = SerialPort::try_new(COM1).unwrap();

    writeln!(
        &mut logger,
        "vector {} called, but no handler is registered in the kernel for it",
        vector
    )
    .ignore();

    Apic::signal_eoi();
}

static mut IV_HANDLER_FNS: [fn(vector: u64); 224] = [default_handler; 224];

extern "C" {
    pub(crate) fn isr_dummy();
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
        let h = IV_HANDLER_FNS[called_by as usize];
        h(vector);
    }
}

pub fn register_iv_handler(h: fn(vector: u64), vector: u8) {
    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    if vector < 32 {
        panic!("Cannot set vector handler lower than 32");
    }
    unsafe {
        let idx = (vector - 32) as usize;
        IV_HANDLER_FNS[idx] = h;
        logln!("setup handler for {}", idx);
    }
    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
}

global_asm! {
    include_str!("isa_handler.asm"),
}
