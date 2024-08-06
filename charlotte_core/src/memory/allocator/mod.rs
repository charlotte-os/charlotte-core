//! # Allocator
//!
//! The kernel allocator provides a single unified interface for allocating and deallocating memory
//! in the kernel's address space. It is implemented as a bump allocator with holes
//! (https://wiki.osdev.org/Bump_Allocator_with_Holes) and is designed to be simple and fast.

use core::num::NonZeroUsize;
use core::ops::Deref;
use core::ptr::NonNull;
use spin::lazy::Lazy;

use crate::bootinfo::KernelAddressRequest;
use crate::bootinfo::KERNEL_ADDRESS_REQUEST;
use crate::memory::address::*;
use crate::memory::pmm::*;

static KERNEL_HEAP_START: Lazy<VirtualAddress> =
    Lazy::new(|| VirtualAddress::try_from(0x8000_0000_0000).unwrap());

static KERNEL_HEAP_END: Lazy<VirtualAddress> = Lazy::new(|| {
    let kaddr_response = KERNEL_ADDRESS_REQUEST
        .get_response()
        .expect("Failed to obtain kernel address from Limine");
    VirtualAddress::try_from(kaddr_response.virtual_base()).unwrap()
});

static KERNEL_HEAP_SIZE: Lazy<usize> =
    Lazy::new(|| ((*KERNEL_HEAP_END).bits() - (*KERNEL_HEAP_START).bits()) as usize);

/// The minimum size of an allocation in the kernel heap.
/// This is currently set to 8 bytes or 64 bits.
static MIN_ALLOC_SIZE: usize = 8;

static ALLOC_BITMAP_SIZE: Lazy<usize> = Lazy::new(|| *KERNEL_HEAP_SIZE / MIN_ALLOC_SIZE);

/// ## The kernel heap allocator
/// The kernel heap allocator is a simple bitmap allocator that allocates memory in the kernel heap
/// in 8 byte (64 bit) chunks. It is designed to be simple and fast and is used to allocate memory
/// for kernel data that needs to be dynamically allocated.
struct CcAlloc;

impl CcAlloc {
    pub fn init() {
        let n_bm_pages = *ALLOC_BITMAP_SIZE / 4096 + 1;
        let frames = PHYSICAL_FRAME_ALLOCATOR
            .lock()
            .allocate_contiguous(n_bm_pages as u64, 4096)
            .expect("Failed to allocate frames for the kernel heap bitmap");
        todo!("Map the kernel heap bitmap at the beginning of the heap");
    }
}
