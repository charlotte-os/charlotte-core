#![no_std]
#![no_main]
#![warn(missing_copy_implementations)]

use core::fmt::Write;

use arch::{Api, ArchApi};
use framebuffer::colors::Color;
use memory::pmm::*;

use crate::framebuffer::framebuffer::FRAMEBUFFER;
use crate::kmon::Kmon;

mod acpi;
mod arch;
mod bootinfo;
mod framebuffer;
mod memory;
mod kmon;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut arch_api = ArchApi::new_arch_api();

    FRAMEBUFFER.lock().clear_screen(Color::BLACK);

    logln!("Initializing BSP");
    ArchApi::init_bsp();
    logln!("BSP Initialized");

    logln!("Initializing System Configuration from ACPI tables");
    let acpi_static_tables = acpi::init_acpi();
    logln!("ACPI derived configuration complete");
    arch_api.init_acpi_tables(&acpi_static_tables);

    logln!("Enable the interrupts");
    arch_api.init_interrupts();
    logln!("Interrupts initialized and enabled");

    logln!(
        "Number of Significant Physical Address Bits Supported: {}",
        ArchApi::get_paddr_width()
    );
    logln!(
        "Number of Significant Virtual Address Bits Supported: {}",
        ArchApi::get_vaddr_width()
    );

    let memory_map = MemoryMap::get();
    logln!(
        "Physical Address Space Size: {} MiB",
        memory_map.total_memory() / 1024 / 1024
    );
    logln!(
        "Total Physical Memory: {} MiB",
        memory_map.usable_memory() / 1024 / 1024
    );

    logln!("Testing Physical Memory Manager");
    logln!("Performing single frame allocation and deallocation test.");
    let alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    let alloc2 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    match alloc {
        Ok(frame) => {
            logln!("Allocated frame with physical base address: {:?}", frame);
            let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
            logln!("Deallocated frame with physical base address: {:?}", frame);
        }
        Err(e) => {
            logln!("Failed to allocate frame: {:?}", e);
        }
    }
    let alloc3 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate();
    logln!("alloc2: {:?}, alloc3: {:?}", alloc2, alloc3);
    let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc2.unwrap());
    let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(alloc3.unwrap());
    logln!("Single frame allocation and deallocation test complete.");
    logln!("Performing contiguous frame allocation and deallocation test.");
    let contiguous_alloc = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(256, 64);
    match contiguous_alloc {
        Ok(frame) => {
            logln!(
                "Allocated physically contiguous region with physical base address: {:?}",
                frame
            );
            let _ = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(frame);
            logln!(
                "Deallocated physically contiguous region with physical base address: {:?}",
                frame
            );
        }
        Err(e) => {
            logln!("Failed to allocate contiguous frames: {:?}", e);
        }
    }
    logln!("Contiguous frame allocation and deallocation test complete.");
    logln!("Physical Memory Manager test suite finished.");

    logln!("All sanity checks passed, initializing kernel Main Loop");

    logln!("Bring up finished, starting kernel interactive prompt");
    let port = ArchApi::get_logger();
    let mut mon = Kmon::new(port);
    mon.repl_loop();
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    logln!("A kernel panic has occurred due to a Rust runtime panic.");
    logln!("PanicInfo: {:?}", _info);
    ArchApi::panic()
}
