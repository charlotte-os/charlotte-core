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
    FRAMEBUFFER.lock().draw_text(100, 100, " !\"#$%&'()*+,-./",0xFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 116, "0123456789:;<=>?",0xFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 132, "@ABCDEFGHIJKLMNO",0xFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 148, "PQRSTUVWXYZ[\\]^_",0xFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 164, "`abcdefghijklmno",0xFFFFFFF);
    FRAMEBUFFER.lock().draw_text(100, 180, "pqrstuvwxyz{:}~\t",0xFFFFFFF);
    
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
