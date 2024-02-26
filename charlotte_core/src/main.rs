#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

mod arch;
mod bootinfo;
mod framebuffer;

use core::fmt::Write;

use arch::{Api, ArchApi};

use framebuffer::console::CONSOLE;

use crate::framebuffer::framebuffer::FRAMEBUFFER;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    FRAMEBUFFER.lock().clear_screen(0x00000000);
    println!("Hello, world!");

    logln!("Initializing BSP");
    ArchApi::init_bsp();
    logln!("BSP Initialized");

    logln!("All tests in main passed.");

    logln!("Number of Significant Physical Address Bits Supported: {}", ArchApi::get_paddr_width());
    logln!("Number of Significant Virtual Address Bits Supported: {}", ArchApi::get_vaddr_width());

    logln!("Testing physical frame allocator");

    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
