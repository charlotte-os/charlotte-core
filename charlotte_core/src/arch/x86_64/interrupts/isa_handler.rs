use crate::arch::x86_64::cpu::{asm_irq_disable, asm_irq_enable, asm_outb};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::serial::ComPort::COM1;
use crate::arch::x86_64::serial::SerialPort;
use crate::memory::address::VirtualAddress;
use core::arch::global_asm;
use core::fmt::Write;
use ignore_result::Ignore;
use crate::logln;

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
    pub fn asm_sti();
    pub fn asm_iretq();
    fn isr_wrapper();
    pub(crate) fn isr_dummy();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IntIdx {
    Timer = 0x20,
}

#[no_mangle]
pub static mut TIMER_CALLED_TIMES: u64 = 0;

pub fn load_dummy_handlers(idt: &mut Idt) {
    for gate in 32..256 {
        idt.set_gate(gate, isr_dummy, 1 << 3, true, true);
    }
    idt.load();
}

pub fn set_isr(idt: &mut Idt, gate: usize, isr_ptr: unsafe extern "C" fn()) {
    idt.set_gate(gate, isr_ptr, 1 << 3, true, true);
    idt.load();
}

#[no_mangle]
pub extern "C" fn timer_handler() {
    unsafe { TIMER_CALLED_TIMES += 1 }

    let mut logger = SerialPort::try_new(COM1).unwrap();

    writeln!(&mut logger, "{}", unsafe { TIMER_CALLED_TIMES }).ignore();

    hello();
}

fn hello() {
    logln!(".");
}

global_asm! {
    include_str!("isa_handler.asm"),
}
