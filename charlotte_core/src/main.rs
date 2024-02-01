#![no_std]
#![no_main]

mod arch;

use core::arch::asm;
use core::fmt::Write;

use arch::{Api, ArchApi};

// Set the limine revision to 1
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(1);

// Request a framebuffer from Limine
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();

    // Check if we have a framebuffer, and halt if we don't
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            ArchApi::halt();
        }

        // Get the first framebuffer's information
        let framebuffer = &framebuffer_response.framebuffers()[0];

        write!(&mut logger, "Framebuffer located at: {:p}\n", framebuffer.address.as_ptr().unwrap());
    }

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
