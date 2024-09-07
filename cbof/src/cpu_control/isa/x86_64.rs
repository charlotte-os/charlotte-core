///! x86_64 implementation of the CPU control interface
use core::arch::asm;

pub struct MSRValue {
    pub eax: u32,
    pub edx: u32,
}

#[inline(always)]
pub extern "C" fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}
#[inline(always)]
pub extern "C" fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}
#[inline(always)]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

pub fn read_msr(selector: u32) -> MSRValue {
    let mut eax: u32;
    let mut edx: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") selector,
            out("eax") eax,
            out("edx") edx,
        );
    }

    MSRValue { eax, edx }
}

pub fn write_msr(selector: u32, value: MSRValue) {
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") selector,
            in("eax") value.eax,
            in("edx") value.edx,
        );
    }
}
