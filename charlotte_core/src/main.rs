#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

mod acpi;
mod arch;
mod bootinfo;
mod framebuffer;

use core::fmt::Write;

use arch::x86_64::memory::pmm::PFA;
use arch::{Api, ArchApi};

use framebuffer::colors::Color;
use framebuffer::console::CONSOLE;

use crate::framebuffer::framebuffer::FRAMEBUFFER;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    FRAMEBUFFER.lock().clear_screen(Color::BLACK);
    println!("Hello, world!");

    logln!("Initializing BSP");
    ArchApi::init_bsp();
    logln!("BSP Initialized");

    logln!("Initializing ACPI");
    acpi::init_acpi();

    logln!("ACPI Initialized");

    logln!("All tests in main passed.");

    logln!(
        "Number of Significant Physical Address Bits Supported: {}",
        ArchApi::get_paddr_width()
    );
    logln!(
        "Number of Significant Virtual Address Bits Supported: {}",
        ArchApi::get_vaddr_width()
    );

    logln!("Testing physical frame allocator");
    let mut pfa = PFA.lock();
    match pfa.allocate_frames(50, None) {
        Ok(region_descriptor) => {
            logln!("Allocated region: {:?}", region_descriptor);
            let _ = pfa.deallocate_frames(region_descriptor);
            logln!("Deallocated previously allocated region.");
        }
        Err(e) => {
            logln!("Failed to allocate region with error {:?}", e);
        }
    }

    CONSOLE.lock().set_colors(Color::RED, Color::WHITE);
    println!("Example of setting console colors");
    CONSOLE.lock().set_colors(Color::WHITE, Color::BLACK);
    println!(
        "Example println with [b{:X}color [f{:X}[b{:X}formatting",
        Color::BLUE,
        Color::GREEN,
        Color::MAGENTA
    );

    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
