mod page_frame_allocator;

use core::arch::x86_64::__cpuid_count;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PADDR_SIG_BITS: u8 = {
        let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
        // 0x80000008 is the highest cpuid leaf that returns the physical address width in EAX[7:0]
        let psig_bits = cpuid.eax & 0xFF;
        psig_bits as u8
    };
    pub static ref VADDR_SIG_BITS: u8 = {
        let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
        // 0x80000008 is the highest cpuid leaf that returns the virtual address width in EAX[15:8]
        let vsig_bits = (cpuid.eax >> 8) & 0xFF;
        vsig_bits as u8
    };
}
