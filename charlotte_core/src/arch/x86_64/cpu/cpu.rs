#![feature(asm)]

pub fn asm_halt() -> ! {
    unsafe {
        asm!(
            "cli",
            "hlt",
            "jmp asm_halt",
            options(noreturn)
        );
    }
}

pub fn asm_inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
        );
    }
    value
}

pub fn asm_outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

pub fn asm_read_msr(msr: u32) -> (u32, u32) {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
        );
    }
    (low, high)
}

pub fn asm_write_msr(msr: u32, low: u32, high: u32) {
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
        );
    }
}

pub fn asm_get_privilege_level() -> u32 {
    let privilege_level: u32;
    unsafe {
        asm!(
            "mov eax, cs",
            "and eax, 3",
            out("eax") privilege_level,
        );
    }
    privilege_level
}

pub fn asm_get_vendor_string(buffer: &mut [u8; 12]) {
    unsafe {
        asm!(
            "mov r10, rbx",        // preserve rbx
            "xor eax, eax",        // eax = 0
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "mov rbx, r10",        // restore rbx
            in("rdi") buffer.as_mut_ptr(),
            out("eax") _,          // clobbered registers
            out("ecx") _,
            out("edx") _,
            out("r10") _,
        );
    }
}
