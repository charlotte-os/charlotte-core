#![no_std]
#![no_main]

mod arch;

use core::arch::asm;
use core::fmt::Write;
use static_alloc::Bump;

use arch::{Api, ArchApi};

struct FrameAllocator {
    // Using u8 for each frame, where the higher 4 bits represent reference count
    // and the lower 4 bits represent allocation status
    frames: [u8; 256],
}

impl FrameAllocator {
    fn new() -> Self {
        Self {
            frames: [0; 256],
        }
    }

    fn allocate_frame(&mut self) -> Option<usize> {
        for (frame_index, frame) in self.frames.iter_mut().enumerate() {
            let allocation_status = frame & 0x0F;

            if allocation_status == 0 {
                *frame = 0x11; // Mark frame as allocated with reference count 1
                return Some(frame_index);
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: usize) {
        if frame < self.frames.len() {
            let frame_byte = &mut self.frames[frame];
            let allocation_status = *frame_byte & 0x0F;

            if allocation_status != 0 {
                *frame_byte &= 0xF0; // Clear the lower 4 bits
                *frame_byte += 0x10; // Increase reference count by 1
            }
        }
    }

    fn allocate_contiguous_frames(&mut self, count: usize) -> Option<usize> {
        let mut consecutive_free_frames = 0;
        let mut first_frame = None;

        for (frame_index, frame) in self.frames.iter_mut().enumerate() {
            let allocation_status = frame & 0x0F;

            if allocation_status == 0 {
                consecutive_free_frames += 1;

                if consecutive_free_frames == 1 {
                    first_frame = Some(frame_index);
                }

                if consecutive_free_frames == count {
                    for i in 0..count {
                        let current_frame = first_frame.unwrap() + i;
                        self.frames[current_frame] = 0x11; // Mark frame as allocated with reference count 1
                    }

                    return first_frame;
                }
            } else {
                // Frame is not free, reset consecutive count
                consecutive_free_frames = 0;
            }
        }

        // No contiguous block found
        None
    }

    fn deallocate_contiguous_frames(&mut self, start_frame: usize, count: usize) {
        for i in 0..count {
            let current_frame = start_frame + i;
            if current_frame < self.frames.len() {
                let frame_byte = &mut self.frames[current_frame];
                let allocation_status = *frame_byte & 0x0F;

                if allocation_status != 0 {
                    *frame_byte &= 0xF0; // Clear the lower 4 bits
                    *frame_byte += 0x10; // Increase reference count by 1
                }
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
