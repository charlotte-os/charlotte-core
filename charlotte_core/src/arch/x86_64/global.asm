.code64

.bss
bsp_regs:
    .skip 128 // Skip the size in bytes for saved registers [rip + u64;16]

.text
.global save_regs
save_regs:
        mov [rip + bsp_regs + 0 * 8], rax
        mov [rip + bsp_regs + 1 * 8], rbx
        mov [rip + bsp_regs + 2 * 8], rcx
        mov [rip + bsp_regs + 3 * 8], rdx
        mov [rip + bsp_regs + 4 * 8], rsi
        mov [rip + bsp_regs + 5 * 8], rdi
        mov [rip + bsp_regs + 6 * 8], rbp
        mov [rip + bsp_regs + 7 * 8], r8
        mov [rip + bsp_regs + 8 * 8], r9
        mov [rip + bsp_regs + 9 * 8], r10
        mov [rip + bsp_regs + 10 * 8], r11
        mov [rip + bsp_regs + 11 * 8], r12
        mov [rip + bsp_regs + 12 * 8], r13
        mov [rip + bsp_regs + 13 * 8], r14
        mov [rip + bsp_regs + 14 * 8], r15
        ret

.global restore_regs
restore_regs:
        mov rax, [rip + bsp_regs + 0 * 8]
        mov rbx, [rip + bsp_regs + 1 * 8]
        mov rcx, [rip + bsp_regs + 2 * 8]
        mov rdx, [rip + bsp_regs + 3 * 8]
        mov rsi, [rip + bsp_regs + 4 * 8]
        mov rdi, [rip + bsp_regs + 5 * 8]
        mov rbp, [rip + bsp_regs + 6 * 8]
        mov r8, [rip + bsp_regs + 7 * 8]
        mov r9, [rip + bsp_regs + 8 * 8]
        mov r10, [rip + bsp_regs + 9 * 8]
        mov r11, [rip + bsp_regs + 10 * 8]
        mov r12, [rip + bsp_regs + 11 * 8]
        mov r13, [rip + bsp_regs + 12 * 8]
        mov r14, [rip + bsp_regs + 13 * 8]
        mov r15, [rip + bsp_regs + 14 * 8]
        ret
