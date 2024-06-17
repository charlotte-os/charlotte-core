#![feature(asm)]

pub fn asm_load_idt(idt: &DescriptorTablePointer) {
    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) idt,
            options(nostack, preserves_flags),
        );
    }
}

#[repr(C, packed)]
pub struct DescriptorTablePointer {
    limit: u16,
    base: u64,
}
