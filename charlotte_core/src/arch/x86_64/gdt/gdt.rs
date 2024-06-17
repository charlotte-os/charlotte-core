#![feature(asm)]

#[repr(C, packed)]
struct Gdtr {
    limit: u16,
    base: u64,
}

static mut GDTR: Gdtr = Gdtr {
    limit: (64 * 7 - 1) as u16,
    base: 0,
};

pub fn asm_load_gdt(gdt_base: u64) {
    unsafe {
        GDTR.base = gdt_base;
        asm!(
            "lgdt [{0}]",
            in(reg) &GDTR,
        );
    }
}

pub fn asm_reload_segment_regs() {
    unsafe {
        asm!(
            "mov rax, 1",       // Segment descriptor 1 is the kernel code segment
            "shl rax, 3",
            "push rax",
            "lea rax, [rip + reload_cs]",
            "push rax",
            "retfq",
            "reload_cs:",
            "mov ax, 2",        // Segment descriptor 2 is the kernel data segment
            "shl ax, 3",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "ret",
            options(noreturn)
        );
    }
}

pub fn asm_load_tss() {
    unsafe {
        asm!(
            "mov ax, 5",        // Segment descriptor 5 is the TSS
            "shl ax, 3",
            "ltr ax",
        );
    }
}
