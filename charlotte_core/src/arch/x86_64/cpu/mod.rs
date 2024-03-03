use core::arch::x86_64::__cpuid_count;
use spin::lazy::Lazy;

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
}
