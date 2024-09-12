#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]
#![feature(naked_functions)]

mod boot_info;
pub mod common;
mod cpu_control;
mod framebuffer;
mod graphics;
mod interrupts;
mod logging;
mod memory;
mod monitor;
// Only include  and run the self-test batteries in debug builds
#[cfg(debug_assertions)]
mod self_test_batteries;
mod uart;

/// # The kernel entry point
/// This function is the entry point for the kernel. It is called by the bootloader.
/// ## Returns
/// This function never returns since it halts the CPU and eventually will
/// hand the BSP off to the scheduler once it finishes initializing the system.
#[no_mangle]
pub extern "C" fn main() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `main` because of the linker script.
    logln!("Entered cbof!");
    /*Intialize the system here*/

    // halt the BSP
    // This is a placeholder until a thread scheduler is implemented
    // Once a scheduler is available the BSP will be handed off to it at this point
    cpu_control::halt()
}

/// # The panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    //TODO: Halt all LPs and print panic message
    cpu_control::halt()
}
