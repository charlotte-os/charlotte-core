use idt::BSP_IDT;

mod exceptions;
mod idt;

pub fn init_interrupts() {
    exceptions::load_exceptions(&mut idt::BSP_IDT.lock());
    BSP_IDT.lock().load(); // lock and load lol
}