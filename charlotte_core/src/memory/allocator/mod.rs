//! # Allocator
//!
//! The kernel allocator provides a single unified interface for allocating and deallocating memory
//! in the kernel's address space.
extern crate alloc;


mod page_level;


use alloc::alloc::{GlobalAlloc, Layout};

use spin::lazy::Lazy;


use crate::bootinfo::KERNEL_ADDRESS_REQUEST;
use crate::memory::address::*;

static KERNEL_HEAP_START: Lazy<VirtualAddress> =
    Lazy::new(|| VirtualAddress::try_from(0x8000_0000_0000usize).unwrap());

static KERNEL_HEAP_END: Lazy<VirtualAddress> = Lazy::new(|| {
    let kaddr_response = KERNEL_ADDRESS_REQUEST
        .get_response()
        .expect("Failed to obtain kernel address from Limine");
    VirtualAddress::try_from(kaddr_response.virtual_base()).unwrap()
});

/// The kernel allocator.
/// This allocator is used to allocate memory in the kernel's address space.
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}


