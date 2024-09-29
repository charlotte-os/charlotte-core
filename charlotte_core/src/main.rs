//! # The CBOF kernel

//compiler directives
#![no_std]
#![no_main]
//lints
#![warn(missing_copy_implementations)]
#![deny(missing_docs)]
//experimental features
#![feature(cfg_match)]
#![feature(naked_functions)]

mod boot_info;
pub mod common;
mod cpu_control;
mod graphics;
mod init;
mod interrupts;
mod logging;
mod memory;
mod monitor;
// Only include  and run the self-test batteries in debug builds
#[cfg(debug_assertions)]
mod self_test_batteries;
mod uart;

use init::InitApi;

/// # The kernel entry point
/// This function is the entry point for the kernel. It is called by the bootloader.
/// ## Returns
/// This function never returns since it halts the CPU and eventually will
/// hand the BSP off to the scheduler once it finishes initializing the system.
#[no_mangle]
pub extern "C" fn main() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `main` because of the linker script.
    logln!("Entering Charlotte Core!");
    /*Intialize the system*/
    logln!("Initializing system...");
    init::InitApiImpl::init_system();
    logln!("System initialized!");
    /*Hand the BSP off to the scheduler here*/

    // halt the BSP
    // This is a placeholder until a thread scheduler is implemented
    // Once a scheduler is available the BSP will be handed off to it at this point
    logln!("Reached the end of kernel main. Halting BSP.");
    cpu_control::halt()
}

/// # The panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        logln!("panic occurred in file '{}' at line {} with massage: '{}'",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        logln!("panic occurred from an unknown location with message: '{}'",
            info.message()
        );
    }
    cpu_control::halt()
}
