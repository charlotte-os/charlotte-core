.section .bss
.align 8
.lcomm bsp_regs, 64 * 16

.section .text
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

.global isr_divide_by_zero
isr_divide_by_zero:
        call save_regs
        call ih_divide_by_zero
        call restore_regs
        iretq

.global isr_double_fault
isr_double_fault:
        /*Registers are not saved since this exception is an abort*/
        pop rdi //pop the error code (should always be 0)
        call ih_double_fault
        hlt //halt the core since double faults are an abort

.global isr_general_protection_fault
isr_general_protection_fault:
        call save_regs
        pop rdi // pop the error code
        call ih_general_protection_fault
        call restore_regs
        iretq

.global isr_page_fault
isr_page_fault:
        call save_regs
        pop rdi // pop the error code
        call ih_page_fault
        call restore_regs
        iretq