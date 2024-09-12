///! x86_64 implementation of the CPU control interface
use core::arch::asm;

#[inline(always)]
pub extern "C" fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}
#[inline(always)]
pub extern "C" fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}
#[inline(always)]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
