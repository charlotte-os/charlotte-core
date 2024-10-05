//! # Allocator
//!
//! The kernel allocator provides a single unified interface for allocating and deallocating memory
//! in the kernel's address space.

use alloc::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;
use core::num::NonZeroUsize;
use core::ops::Deref;
use core::ptr::NonNull;
use spin::lazy::Lazy;

use crate::arch::{self, Api, ArchApi, MemoryMap, Api::MemoryMap};
use crate::bootinfo::KernelAddressRequest;
use crate::bootinfo::KERNEL_ADDRESS_REQUEST;
use crate::memory::address::*;
use crate::memory::pmm::*;

use super::pmm;

static KERNEL_HEAP_START: Lazy<VirtualAddress> =
    Lazy::new(|| VirtualAddress::try_from(0x8000_0000_0000).unwrap());

static KERNEL_HEAP_END: Lazy<VirtualAddress> = Lazy::new(|| {
    let kaddr_response = KERNEL_ADDRESS_REQUEST
        .get_response()
        .expect("Failed to obtain kernel address from Limine");
    VirtualAddress::try_from(kaddr_response.virtual_base()).unwrap()
});

/// The kernel allocator.
/// This allocator is used to allocate memory in the kernel's address space.
pub struct Allocator;

impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

pub struct PageAllocator;

impl PageAllocator {
    pub fn allocate(size: NonZeroUsize, alignment: NonZeroUsize) -> Result<VirtualAddress, Error> {
        // find a virtual address range to map the pages to
        let vbase: VirtualAddress = Api::MemoryMap.find_free_range(size, alignment)?;
        Api::MemoryMap.map_pages(vbase, size, Api::MemoryMap::get_flags(crate::arch::MemType::KernelReadWrite))?;
        Ok(vbase)
    }

    pub fn deallocate(paddr: PhysicalAddress, size: NonZeroUsize) -> Result<(), Error> {
        todo!()
    }
}

