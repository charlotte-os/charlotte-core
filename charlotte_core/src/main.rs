#![no_std]
#![no_main]

mod arch;

use core::arch::asm;
use core::fmt::Write;
use static_alloc::Bump;

use arch::{Api, ArchApi};

// Adjust the buffer size if necessary
#[global_allocator]
static ALLOCATOR: Bump<[u8; 8192]> = Bump::uninit();

struct FrameAllocator {
    bitmap: [u8; 256] // Also can adjust this
}

impl FrameAllocator {
    fn new() -> Self {
        Self { bitmap: [0; 256] }
    }

    fn allocate_frame(&mut self) -> Option<usize> {
        for (byte_index, byte) in self.bitmap.iter_mut().enumerate() {
            if *byte != 0xFF {
                for bit_index in 0..8 {
                    let mask = 1 << bit_index;

                    if (*byte && mask) == 0 {
                        *byte |= mask;
                        return Some(byte_index * 8 + bit_index);
                    }
                }
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: usize) {
        let byte_index = frame / 8;
        let bit_index = frame % 8;
        let mask = !(1 << bit_index);
        self.bitmap[byte_index] &= mask;
    }
}

static mut FRAME_ALLOCATOR: Option<FrameAllocator> = None;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();
    write!(&mut logger, "Initializing BSP\n").unwrap();
    ArchApi::init_bsp();
    write!(&mut logger, "BSP Initialized\n").unwrap();

    FRAME_ALLOCATOR.replace(FrameAllocator::new());

    if let Some(allocated_frame) = FRAME_ALLOCATOR.as_mut().unwrap().allocate_frame() {
        write!(&mut logger, "Allocated frame at physical address: 0x{:X}\n", allocated_frame).unwrap();

        // Do whatever

        // Time to deallocate
        FRAME_ALLOCATOR.as_mut().unwrap().deallocate_frame(allocated_frame);
        write!(&mut logger, "Deallocated frame at physical address: 0x{:X\n}", allocated_frame).unwrap();
    } else {
        write!(&mut logger, "Frame allocation failed.\n").unwrap();
    }

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
