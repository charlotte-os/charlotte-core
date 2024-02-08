//! # Memory Management Subsystem
//! The memory management subsystem is responsible for managing the direct mapping of physical 
//! memory in the kernel's address space, allocating and deallocating physical frames, and managing
//! all virtual address spaces.

mod pmm;
mod vmm;

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
}

