use core::num::NonZeroUsize;

use crate::arch::{ArchApi, Api, MemoryMap, MemType};
use crate::memory::{
    address::{*, PhysicalAddress, VirtualAddress},
    pmm::PHYSICAL_FRAME_ALLOCATOR
};

enum Error {
    OutOfMemory,
    AlignmentUnavailable,
    InvalidAlignment,
    InvalidSize,
    InvalidBase,
    ArchMemoryMapError(<<ArchApi as Api>::MemoryMap as MemoryMap>::Error),
    PmmError(crate::memory::pmm::Error),
}
// This is here to allow propagating up the error type from the architecture-specific memory map
impl From<<<ArchApi as Api>::MemoryMap as MemoryMap>::Error> for Error {
    fn from(error: <<ArchApi as Api>::MemoryMap as MemoryMap>::Error) -> Self {
        Error::ArchMemoryMapError(error)
    }
}
// This is here to allow propagating up the error type from the physical memory manager
impl From<crate::memory::pmm::Error> for Error {
    fn from(error: crate::memory::pmm::Error) -> Self {
        Error::PmmError(error)
    }
}

pub struct PageLevelAllocator;

impl PageLevelAllocator {
    pub fn allocate(size: NonZeroUsize, alignment: NonZeroUsize) -> Result<VirtualAddress, Error> {
        let corrected_alignment = if alignment.get() < PAGE_SIZE {
            // will never panic because PAGE_SIZE is a non-zero constant
            NonZeroUsize::new(PAGE_SIZE).unwrap()
        } else {
            alignment
        };
        // find a virtual address range to map the pages to
        let vbase: VirtualAddress = ArchApi::get_memory_map().find_available_region(size, alignment.get(), *super::KERNEL_HEAP_START, *super::KERNEL_HEAP_END)?;
        // map the pages to the virtual address range
        let mut remaining_size: usize = size.get();
        let mut remaining_vbase: VirtualAddress = vbase;
        // only use huge pages if the requested allocation is suitably aligned
        if corrected_alignment.get() % PAGE_SIZE * 512 * 512 == 0 {
            let n_huge_pages = remaining_size / (PAGE_SIZE * 512 * 512);
            remaining_size %= PAGE_SIZE * 512 * 512;
            for _ in 0..n_huge_pages {
                let paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(PAGE_SIZE * 512 * 512, corrected_alignment.get())?;
                ArchApi::get_memory_map().map_huge_page(remaining_vbase, paddr, <ArchApi as crate::arch::Api>::MemoryMap::get_flags(MemType::KernelReadWrite))?;
                remaining_vbase += PAGE_SIZE * 512 * 512;
            }
        }
        // only use large pages if the requested allocation is suitably aligned
        if corrected_alignment.get() % PAGE_SIZE * 512 == 0 {
            let n_large_pages = remaining_size / (PAGE_SIZE * 512);
            remaining_size %= PAGE_SIZE * 512;
            for _ in 0..n_large_pages {
                let paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(PAGE_SIZE * 512, corrected_alignment.get())?;
                ArchApi::get_memory_map().map_large_page(remaining_vbase, paddr, <ArchApi as crate::arch::Api>::MemoryMap::get_flags(MemType::KernelReadWrite))?;
                remaining_vbase += PAGE_SIZE * 512;
            }
        }
        // we always correct the alignment to be at least PAGE_SIZE thus we can always use standard pages
        let n_pages = if remaining_size % PAGE_SIZE == 0 {
            remaining_size / PAGE_SIZE
        } else {
            remaining_size / PAGE_SIZE + 1
        };
        for _ in 0..n_pages {
            let paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate()?;
            ArchApi::get_memory_map().map_page(remaining_vbase, paddr, <ArchApi as crate::arch::Api>::MemoryMap::get_flags(MemType::KernelReadWrite))?;
            remaining_vbase += PAGE_SIZE;
        }

        Ok(vbase)
    }

    pub fn deallocate(base: VirtualAddress, size: NonZeroUsize) -> Result<(), Error> {
        todo!()
    }
}
