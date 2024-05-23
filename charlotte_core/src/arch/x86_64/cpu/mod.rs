use core::{arch::x86_64::__cpuid_count, fmt::Write};
use spin::lazy::Lazy;

use crate::logln;

/// The number of significant bits in a physical address on the current CPU.
pub static PADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
    let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
    // 0x80000008 is the highest cpuid leaf that returns the physical address width in EAX[7:0]
    let psig_bits = cpuid.eax & 0xFF;
    psig_bits as u8
});

/// The number of significant bits in a virtual address on the current CPU.
pub static VADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
    let cpuid = unsafe { __cpuid_count(0x80000008, 0) };
    // 0x80000008 is the highest cpuid leaf that returns the virtual address width in EAX[15:8]
    let vsig_bits = (cpuid.eax >> 8) & 0xFF;
    vsig_bits as u8
});

extern "C" {
    pub fn asm_halt() -> !;
    pub fn asm_inb(port: u16) -> u8;
    pub fn asm_outb(port: u16, val: u8);
    pub fn asm_get_vendor_string(dest: &mut [u8; 12]);
    pub fn asm_read_msr(msr: u32, lo: &mut u32, hi: &mut u32);
    pub fn asm_write_msr(msr: u32, eax: u32, edx: u32);
    pub fn asm_get_privilege_level() -> u8;
}

pub struct MSRResponse {
    pub eax: u32,
    pub edx: u32,
}

pub fn get_privilege_level() -> u8 {
    unsafe { asm_get_privilege_level() }
}

pub fn assert_msr_presence() -> bool {
    let cpuidresult = unsafe { __cpuid_count(0, 0) };
    return cpuidresult.edx & 1 << 5 != 0;
}

pub fn read_msr(msr: u32) -> MSRResponse {
    if !assert_msr_presence() {
        panic!("Processor lacks msr support and read_msr was called!");
    }
    let mut lo = 0;
    let mut hi = 0;
    unsafe { asm_read_msr(msr, &mut lo, &mut hi) }

    MSRResponse { eax: lo, edx: hi }
}

pub fn write_msr(msr: u32, eax: u32, edx: u32) {
    if get_privilege_level() != 0 {
        logln!("Privilege level is not 0, is {}", get_privilege_level());
        return;
    }
    if !assert_msr_presence() {
        panic!("Processor lacks msr support and write_msr was called!");
    }
    logln!("Writing {:X}, {:X} to MSR[{:X}]", eax, edx, msr);
    unsafe { asm_write_msr(msr, eax, edx) };
}
