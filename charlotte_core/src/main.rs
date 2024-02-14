#![no_std]
#![no_main]
#![feature(slice_internals)]

mod access_control;
mod arch;
mod bootinfo;
mod framebuffer;

use core::arch::asm;
use core::fmt::Write;

use arch::{Api, ArchApi};
use arch::x86_64::memory::pmm::PFA;

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

    writeln!(&mut logger, "Testing physical frame allocator").unwrap();
    let mut pfa = PFA.lock();
    match pfa.allocate_frames(50, None) {
        Ok(region_descriptor) => {
            writeln!(&mut logger, "Allocated region: {:?}", region_descriptor).unwrap();
            pfa.deallocate_frames(region_descriptor);
            writeln!(&mut logger, "Deallocated previously allocated region.").unwrap();
        }
        Err(e) => {
            writeln!(&mut logger, "Failed to allocate region with error {:?}", e).unwrap();
        }
    }
    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    ArchApi::get_logger().write_fmt(format_args!("A kernel panic has occurred due to a Rust runtime panic.\n PanicInfo: {:?}\n", _info)).unwrap();
    println!("A kernel panic has occurred due to a Rust runtime panic.\n PanicInfo: {:?}\n", _info);
    ArchApi::panic()
}
