#![no_std]
#![no_main]

mod arch;

use core::arch::asm;
use core::fmt::Write;
use static_alloc::Bump;

use arch::{Api, ArchApi};

struct FrameAllocator {
    bitmap: [u8; 256], // Adjust this size if necessary
    reference_count: [u8; 256],
}

impl FrameAllocator {
    fn new() -> Self {
        Self {
            bitmap: [0; 256],
            reference_count: [0; 256],
        }
    }

    fn allocate_frame(&mut self) -> Option<usize> {
        for (byte_index, byte) in self.bitmap.iter_mut().enumerate() {
            if *byte != 0xFF {
                for bit_index in 0..8 {
                    let mask = 1 << bit_index;

                    if (*byte && mask) == 0 {
                        *byte |= mask;
                        self.reference_count[byte_index * 8 + bit_index] = 1; // Initialize reference count
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

        if self.reference_count[frame] > 0 {
            self.reference_count[frame] -= 1;

            // If reference count becomes zero, mark the frame as free
            if self.reference_count[frame] == 0 {
                self.bitmap[byte_index] &= mask;
            }
        }
    }

    // This is... not an ideal amount of conditions and loops. I will probably rewrite a bit of this and shorten it up soon. Keeping it for the PR.
    fn allocate_contiguous_frames(&mut self, count: usize) -> Option<usize> {
        let mut consecutive_free_frames = 0;
        let mut first_frame = None;

        for (byte_index, byte) in self.bitmap.iter_mut().enumerate() {
            for bit_index in 0..8 {
                let mask = 1 << bit_index;

                if (*byte & mask) == 0 {
                    // Frame is free
                    consecutive_free_frames += 1;

                    if consecutive_free_frames == 1 {
                        // Mark the start of a potential contiguous block
                        first_frame = Some(byte_index * 8 + bit_index);
                    }

                    if consecutive_free_frames == count {
                        // Found a contiguous block
                        for i in 0..count {
                            if let Some(frame) = first_frame.map(|f| f + i) {
                                *self.reference_count.get_mut(frame).unwrap() = 1; // Initialize reference count
                                *self.bitmap.get_mut(frame / 8).unwrap() |= 1 << (frame % 8); // Mark frame as allocated
                            }
                        }

                        return first_frame;
                    }
                } else {
                    // Frame is not free, reset consecutive count
                    consecutive_free_frames = 0;
                }
            }
        }

        // No contiguous block found
        None
    }

    fn deallocate_contiguous_frames(&mut self, start_frame: usize, count: usize) {
        for i in 0..count {
            if let Some(frame) = start_frame.checked_add(i) {
                self.deallocate_frame(frame);
            }
        }
    }
}

static mut FRAME_ALLOCATOR: Option<FrameAllocator> = None;

// Example architecture-independent allocation interface
pub fn allocate_frames(count: usize) -> Option<usize> {
    // Call the appropriate architecture-specific allocator
    unsafe {
        FRAME_ALLOCATOR.as_mut().unwrap().allocate_contiguous_frames(count)
    }
}

pub fn deallocate_frames(start_frame: usize, count: usize) {
    // Call the appropriate architecture-specific deallocator
    unsafe {
        FRAME_ALLOCATOR.as_mut().unwrap().deallocate_contiguous_frames(start_frame, count);
    }
}

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mut logger = ArchApi::get_logger();
    write!(&mut logger, "Initializing BSP\n").unwrap();
    ArchApi::init_bsp();
    write!(&mut logger, "BSP Initialized\n").unwrap();

    // Initialize the frame allocator
    FRAME_ALLOCATOR.replace(FrameAllocator::new());

    // Example usage of the frame allocator
    if let Some(allocated_frame) = FRAME_ALLOCATOR.as_mut().unwrap().allocate_frame() {
        write!(&mut logger, "Allocated frame at physical address: 0x{:X}\n", allocated_frame).unwrap();

        // Do whatever

        // Time to deallocate
        FRAME_ALLOCATOR.as_mut().unwrap().deallocate_frame(allocated_frame);
        write!(&mut logger, "Deallocated frame at physical address: 0x{:X}\n", allocated_frame).unwrap();
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

    // Example usage of allocating and deallocating contiguous frames
    if let Some(start_frame) = allocate_frames(3) {
        write!(&mut logger, "Allocated contiguous frames starting at address: 0x{:X}\n", start_frame).unwrap();

        // write!(&mut logger, "Testing GP fault\n").unwrap();
        // asm!("int 13");
        // write!(&mut logger, "GP fault test passed\n").unwrap();

        // Deallocate the contiguous frames when done
        deallocate_frames(start_frame, 3);
        write!(&mut logger, "Deallocated contiguous frames starting at address: 0x{:X}\n", start_frame).unwrap();
    } else {
        write!(&mut logger, "Contiguous frame allocation failed.\n").unwrap();
    }

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
