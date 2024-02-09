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

use framebuffer::framebuffer::{FRAMEBUFFER, Point};

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();
    
    FRAMEBUFFER.lock().clear_screen(0x00000000);
    FRAMEBUFFER.lock().draw_text(100, 100, "ABCDEFGHIJKLMNOPQRS \n \nTUVWXYZ", 0xFFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 150, "1234567890", 0xFFFFFFFF);
    FRAMEBUFFER.lock().draw_rect(100, 150, 600, 100, 0xFFFFFFFF);
    FRAMEBUFFER.lock().draw_triangle(Point { x: 250, y: 600 }, Point { x: 300, y: 500 }, Point { x: 350, y: 600 }, 0xFFFFFFFF);
    

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
