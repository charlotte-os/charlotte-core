#![no_std]
#![no_main]

#![feature(slice_internals)]

mod access_control;
mod arch;
mod framebuffer;
mod bootinfo;

use core::arch::asm;
use core::fmt::Write;

use arch::{Api, ArchApi};

use framebuffer::console::CONSOLE;

use crate::framebuffer::framebuffer::FRAMEBUFFER;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();

    FRAMEBUFFER.lock().clear_screen(0x00000000);
    println!("Hello, world!");

    write!(&mut logger, "Initializing BSP\n").unwrap();
    ArchApi::init_bsp();
    write!(&mut logger, "BSP Initialized\n").unwrap();

    write!(&mut logger, "All tests in main passed.\n").unwrap();

    writeln!(
        &mut logger,
        "Number of Significant Physical Address Bits Supported: {}",
        ArchApi::get_paddr_width()
    )
    .unwrap();

    writeln!(
        &mut logger,
        "Number of Significant Virtual Address Bits Supported: {}",
        ArchApi::get_vaddr_width()
    )
    .unwrap();

    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    ArchApi::panic()
}
