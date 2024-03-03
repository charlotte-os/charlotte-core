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

    logln!("Testing Physical Memory Manager");
    logln!("Performing single frame allocation and deallocation test.");
    let alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    let alloc2 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    match alloc {
        Ok(frame) => {
            logln!("Allocated frame with physical base address: {:?}", frame);
            PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
            logln!("Deallocated frame with physical base address: {:?}", frame);
        }
        Err(e) => {
            logln!("Failed to allocate frame: {:?}", e);
        }
    }
    let alloc3 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    logln!("alloc2: {:?}, alloc3: {:?}", alloc2, alloc3);
    PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc2.unwrap());
    PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc3.unwrap());
    logln!("Single frame allocation and deallocation test complete.");
    logln!("Performing contiguous frame allocation and deallocation test.");
    let contiguous_alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(256, 64);
    match contiguous_alloc {
        Ok(frame) => {
            logln!("Allocated physically contiguous region with physical base address: {:?}", frame);
            PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
            logln!("Deallocated physically contiguous region with physical base address: {:?}", frame);
        }
        Err(e) => {
            logln!("Failed to allocate contiguous frames: {:?}", e);
        }
    }
    logln!("Contiguous frame allocation and deallocation test complete.");
    logln!("Physical Memory Manager test suite finished.");

    logln!("Halting BSP");
    ArchApi::halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
