//! # Memory Management Subsystem
//! The memory management subsystem is responsible for managing the direct mapping of physical 
//! memory in the kernel's address space, allocating and deallocating physical frames, and managing
//! all virtual address spaces.

mod pfa;
mod vmm;

use crate::bootinfo;

use core::arch::x86_64::__cpuid_count;
use lazy_static::lazy_static;

lazy_static! {
    ///The number of significant bits in a physical address on the current CPU.
    pub static ref PADDR_SIG_BITS: u8 = {
        let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
        // 0x80000008 is the highest cpuid leaf that returns the physical address width in EAX[7:0]
        let psig_bits = cpuid.eax & 0xFF;
        psig_bits as u8
    };
    ///The number of significant bits in a virtual address on the current CPU.
    pub static ref VADDR_SIG_BITS: u8 = {
        let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
        // 0x80000008 is the highest cpuid leaf that returns the virtual address width in EAX[15:8]
        let vsig_bits = (cpuid.eax >> 8) & 0xFF;
        vsig_bits as u8
    };
    ///This value represents the base virtual address of the direct mapping of physical memory into
    /// kernelspace. It should have the desired physical address added to it and then be cast to a 
    /// pointer to access the desired physical address.
    pub static ref HHDM_BASE: u64 = bootinfo::HHDM_REQUEST.get_response().unwrap().offset();
}

///This function can be used to obtain a reference to an object of type T that is located at the
/// specified physical address. It is unsafe because it dereferences a raw pointer and assumes
/// that the specified physical address is valid and that an object of type T is located at that
/// address.
pub unsafe fn ref_from_paddr<T>(paddr: u64) -> &'static T {
    let ptr = (paddr + *HHDM_BASE) as *const T;
    &*ptr
}
///This function can be used to obtain a mutable reference to an object of type T that is located at the
/// specified physical address. It is unsafe because it dereferences a raw pointer and assumes
/// that the specified physical address is valid and that an object of type T is located at that
/// address.
pub unsafe fn ref_mut_from_paddr<T>(paddr: u64) -> &'static mut T {
    let ptr = (paddr + *HHDM_BASE) as *mut T;
    &mut *ptr
}
