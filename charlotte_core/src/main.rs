#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

mod arch;
mod bootinfo;
mod framebuffer;
mod memory;

use core::fmt::Write;

use arch::{Api, ArchApi};

use framebuffer::console::CONSOLE;

use crate::framebuffer::framebuffer::FRAMEBUFFER;

use memory::pmm::*;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    FRAMEBUFFER.lock().clear_screen(0x00000000);
    println!("Hello, world!");

    logln!("Initializing BSP");
    ArchApi::init_bsp();
    logln!("BSP Initialized");

    logln!("All tests in main passed.");

    logln!(
        "Number of Significant Physical Address Bits Supported: {}",
        ArchApi::get_paddr_width()
    );
    logln!(
        "Number of Significant Virtual Address Bits Supported: {}",
        ArchApi::get_vaddr_width()
    );

    let memory_map = MemoryMap::get();
    logln!("Physical Address Space Size: {} MiB", memory_map.total_memory() / 1024 / 1024);
    logln!("Total Physical Memory: {} MiB", memory_map.usable_memory() / 1024 / 1024);
    logln!("Available Physical Memory: {} MiB", PHYSICAL_MEMORY_MANAGER.lock().pfa.available_memory().expect("available_memory overflowed usize") / 1024 / 1024);
/*     logln!("Testing physical frame allocator");
    let region = PHYSICAL_MEMORY_MANAGER.lock().pfa.allocate_contiguous(32);
    match region {
        Ok(r) => {
            logln!("Allocated region: {:?}", r);
            logln!("Deallocating region");
            PHYSICAL_MEMORY_MANAGER.lock().pfa.deallocate(r);
        }
        Err(e) => {
            logln!("Failed to allocate region with error: {:?}", e);
        }
    }
    logln!("Physical frame allocator test complete"); */

    logln!("Halting BSP");
    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
