use core::arch::asm;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn save_int_regs() {
    unsafe {
        asm!(
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push rbp",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "ret",
            options(noreturn)
        );
    }
}
#[no_mangle]
#[naked]
pub unsafe extern "C" fn restore_int_regs() {
    unsafe {
        asm!(
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            "ret",
            options(noreturn)
        );
    }
}