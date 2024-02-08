#![no_std]
#![no_main]

mod arch;
mod framebuffer;


use framebuffer::*;
use core::arch::asm;
use core::fmt::Write;

use arch::{Api, ArchApi};

use crate::framebuffer::framebuffer::*;


// Set the limine revision to 1
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(1);



#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();
    

    // Check if we have a framebuffer, and halt if we don't
    match init_framebuffer() {
        Some(framebuffer) => {
            framebuffer.clear_screen(0x00FF00FF); // Example usage: clear the screen with green color
            framebuffer.draw_text(100, 100, "ABCDEFGHIJKLMNOPQRS \n \nTUVWXYZ", 0xFFFFFFFF);
            framebuffer.draw_text(100, 150, "1234567890", 0xFFFFFFFF); // Example usage: draw "ABCD" in white
            framebuffer.draw_rect(100, 200, 600, 100, 0xFFFFFFFF);
            framebuffer.draw_triangle(Point { x: 250, y: 600 }, Point { x: 300, y: 500 }, Point { x: 350, y: 600 }, 0xFFFFFFFF);
            write!(&mut logger, "Framebuffer initialized and drawings made.\n").unwrap();
        },
        None => {
            write!(&mut logger, "Failed to initialize framebuffer.\n").unwrap();
            ArchApi::halt();
        }
    };
    

    write!(&mut logger, "Initializing BSP\n").unwrap();
    ArchApi::init_bsp();
    write!(&mut logger, "BSP Initialized\n").unwrap();

    // write!(&mut logger, "Testing double fault\n").unwrap();
    // asm!("int 8");
    // write!(&mut logger, "Double fault test passed\n").unwrap();

    write!(&mut logger, "Testing divide by zero\n").unwrap();
    asm!("int 0");
    write!(&mut logger, "Divide by zero test passed\n").unwrap();

    // write!(&mut logger, "Testing GP fault\n").unwrap();
    // asm!("int 13");
    // write!(&mut logger, "GP fault test passed\n").unwrap();

    write!(&mut logger, "All tests in main passed.\n").unwrap();

    writeln!(&mut logger, "Number of Significant Physical Address Bits Supported: {}", ArchApi::get_paddr_width())
        .unwrap();
    writeln!(&mut logger, "Number of Significant Virtual Address Bits Supported: {}", ArchApi::get_vaddr_width())
        .unwrap();

    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    ArchApi::panic()
}
