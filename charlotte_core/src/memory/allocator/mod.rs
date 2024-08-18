//! # Allocator
//!
//! The kernel allocator provides a single unified interface for allocating and deallocating memory
//! in the kernel's address space.

use core::num::NonZeroUsize;
use core::ops::Deref;
use core::ptr::NonNull;
use spin::lazy::Lazy;

use crate::arch::x86_64;
use crate::arch::{self, Api, ArchApi, MemoryMap};
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

static KERNEL_HEAP_SIZE: Lazy<usize> =
    Lazy::new(|| ((*KERNEL_HEAP_END).bits() - (*KERNEL_HEAP_START).bits()) as usize);

/// The starting size of the free region list in pages.
/// This is the minimum size of the free region list.
/// The free region list will be reallocated if it grows beyond this size.
const FREE_LIST_MIN_PAGES: usize = 16;

enum Error {
    NullPtrNotAllowed,
    VmmError(<<ArchApi as arch::Api>::MemoryMap as arch::MemoryMap>::Error),
    PmmError(pmm::Error),
}

impl From<<<ArchApi as arch::Api>::MemoryMap as arch::MemoryMap>::Error> for Error {
    fn from(e: <<ArchApi as arch::Api>::MemoryMap as arch::MemoryMap>::Error) -> Self {
        Error::VmmError(e)
    }
}

impl From<pmm::Error> for Error {
    fn from(e: pmm::Error) -> Self {
        Error::PmmError(e)
    }
}

/// A struct that describes a region of memory located directly behind it in the kernel heap.
struct AllocRegion {
    base: VirtualAddress,
    size: usize,
}

/// The kernel heap allocator.
pub struct KernelAllocator {
    heap_base: VirtualAddress,
    heap_size: usize,
    free_list_base: NonNull<AllocRegion>,
    free_list_n_pages: usize,
}

impl KernelAllocator {
    /// Attempts to create a new kernel heap allocator.
    ///
    /// # Returns
    /// * `Ok(Self)` if the allocator was successfully created.
    /// * `Err(Error)` if the allocator could not be created due to a null KERNEL_HEAP_START value.
    fn try_new() -> Result<Self, Error> {
        if let Some(free_list_ptr) = NonNull::new(<*mut AllocRegion>::from(*KERNEL_HEAP_START)) {
            Ok(Self {
                heap_base: *KERNEL_HEAP_START + (4 * PAGE_SIZE),
                heap_size: *KERNEL_HEAP_SIZE,
                free_list_base: free_list_ptr,
                free_list_n_pages: FREE_LIST_MIN_PAGES,
            })
        } else {
            Err(Error::NullPtrNotAllowed)
        }
    }

    /// Allocates non-contiguous page frames and maps them to a contiguous virtual address range.
    fn map_pages_at(&mut self, base: VirtualAddress, n_frames: usize) -> Result<(), Error> {
        let mut pfa = PHYSICAL_FRAME_ALLOCATOR.lock();
        let mut frame = PhysicalAddress::try_from(0).unwrap();

        for i in 0..n_frames as u64 {
            frame = pfa.allocate()?;
            let vaddr = base + (i * PAGE_SIZE);
            // Implement ISA agnostic flag sets to allow mapping of pages for kernel and userspace
            // in an ISA independent way.
            todo!("Implement ISA independent page mapping");
        }
        Ok(())
    }
}
