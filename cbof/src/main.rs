#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]
#![feature(naked_functions)]

mod boot_protocol;
mod cpu_control;
mod graphics;
mod interrupts;
mod memory;
mod monitor;

use cpu_control::{CpuControl, CpuControlIfce};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `main` because of the linker script.

    CpuControlIfce::halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    //! This function is called on panic.
    CpuControlIfce::halt()
    //TODO: Halt all LPs and print panic message
}