#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]
#![feature(naked_functions)]

mod boot_protocol;
pub mod common;
mod cpu_control;
mod graphics;
mod interrupts;
mod memory;
mod monitor;
mod acpi;
// Only include  and run the self-test batteries in debug builds
#[cfg(debug_assertions)]
mod self_test_batteries;
mod uart;

use core::fmt::Write;

/// # The kernel entry point
/// This function is the entry point for the kernel. It is called by the bootloader.
/// ## Returns
/// This function never returns since it halts the CPU and eventually will
/// hand the BSP off to the scheduler once it finishes initializing the system.
#[no_mangle]
pub extern "C" fn main() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `main` because of the linker script.

    // Create a serial port and use it for logging
    let mut com =
        // Safety: The conventional COM1 serial port IO port range is known to be valid on the PC platform
        unsafe {
            uart::SerialPort::try_new(uart::SerialAddr::IoPort(uart::IoPort::new(
                uart::ComPort::COM1 as u16,
            )))
            .unwrap()
        };
    writeln!(com, "Entered cbof").unwrap();

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
