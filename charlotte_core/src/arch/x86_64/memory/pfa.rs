//! # Physical Frame Allocator
//! This module contains the physical frame allocator, which is responsible for managing physical
//! memory frames. It provides an interface for allocating and deallocating physical memory frames and
//! contiguous blocks of frames as well as frames that represent MMIO regions.
//! The PFA can be used to allocate and deallocate frames for use by the kernel and user-space applications.
//! It is capable of allocating and deallocating contiguous blocks of frames, which is useful for things like
//! DMA and certain optimization techniques.

use spin::Mutex;

use crate::access_control::Capability;
use crate::limine;

///This constant represents the base virtual address of the direct mapping of physical memory.
/// It should have the desired physical address added to it and then be cast to a pointer
/// to access the desired physical address.
static HHDM_BASE: Mutex<u64> = Mutex::from(limine::HHDM_REQUEST.get_response().unwrap().offset());

///This function can be used to obtain a reference to an object of type T that is located at the
/// specified physical address. It is unsafe because it dereferences a raw pointer and assumes
/// that the specified physical address is valid and that an object of type T is located at that
/// address.
pub unsafe fn ref_from_paddr<T>(paddr: u64) -> &'static T {
    let hhdm_base = HHDM_BASE.lock();
    let ptr = (paddr + *hhdm_base) as *const T;
    &*ptr
}
///This function can be used to obtain a mutable reference to an object of type T that is located at the
/// specified physical address. It is unsafe because it dereferences a raw pointer and assumes
/// that the specified physical address is valid and that an object of type T is located at that
/// address.
pub unsafe fn mut_ref_from_paddr<T>(paddr: u64) -> &'static mut T {
    let hhdm_base = HHDM_BASE.lock();
    let ptr = (paddr + *hhdm_base) as *mut T;
    &mut *ptr
}


///This enum represents the different types of physical memory regions that the PFA can allocate frames from.
/// It is identical to the defines used by Limine with the exception of PfaReserved, which is used to represent
/// regions of physical memory that are reserved for use by the PFA itself and PfaNull, which is used to represent
/// region descriptors that are not in use.
enum PhysicalMemoryType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    KernelAndModules,
    FrameBuffer,
    PfaReserved,
    PfaNull,
}

struct PhysicalMemoryRegion<'a> {
    key: Option<&'a dyn Capability>,
    base: usize,
    n_frames: usize,
    region_type: PhysicalMemoryType,
}

pub struct PhysicalFrameAllocator {
    region_array_base: usize, // physical base address of the array of physical memory regions array
    region_array_len: usize,  // number of elements in the array of physical memory regions
}
