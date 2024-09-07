///! aarch64 implementation of the CPU control interface

use core::arch::asm;

/// # Enable interrupts
/// This function enables interrupts on the calling LP
#[inline(always)]
pub extern "C" fn enable_interrupts() {
    unsafe {
        // Unmask all DAIF exceptions
        asm!("msr daif, #0");
    }
}
/// # Disable interrupts
/// This function disables all interrupts except NMIs on the calling LP
#[inline(always)]
pub extern "C" fn disable_interrupts() {
    unsafe {
        // Mask all DAIF exceptions
        // This does not affect NMIs (use msr allint, #1 to mask everything)
        // 15 = 0b1111, which sets the mask bits for D, A, I, and F
        asm!("msr daif, #15");
    }
}
/// # Halt the CPU
/// This function halts the LP until an interrupt occurs
#[inline(always)]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            // The WFI instruction is used to enter a low-power state until an interrupt occurs
            // This is the equivalent of the HLT instruction on x86_64
            asm!("wfi");
        }
    }
}