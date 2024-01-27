bits 64

section .bss
align 8
bsp_regs:
resq 16

section .text
save_regs:
        mov [rel bsp_regs + 0 * 8], rax
        mov [rel bsp_regs + 1 * 8], rbx
        mov [rel bsp_regs + 2 * 8], rcx
        mov [rel bsp_regs + 3 * 8], rdx
        mov [rel bsp_regs + 4 * 8], rsi
        mov [rel bsp_regs + 5 * 8], rdi
        mov [rel bsp_regs + 6 * 8], rbp
        mov [rel bsp_regs + 7 * 8], r8
        mov [rel bsp_regs + 8 * 8], r9
        mov [rel bsp_regs + 9 * 8], r10
        mov [rel bsp_regs + 10 * 8], r11
        mov [rel bsp_regs + 11 * 8], r12
        mov [rel bsp_regs + 12 * 8], r13
        mov [rel bsp_regs + 13 * 8], r14
        mov [rel bsp_regs + 14 * 8], r15
        ret
restore_regs:
        mov rax, [rel bsp_regs + 0 * 8]
        mov rbx, [rel bsp_regs + 1 * 8]
        mov rcx, [rel bsp_regs + 2 * 8]
        mov rdx, [rel bsp_regs + 3 * 8]
        mov rsi, [rel bsp_regs + 4 * 8]
        mov rdi, [rel bsp_regs + 5 * 8]
        mov rbp, [rel bsp_regs + 6 * 8]
        mov r8, [rel bsp_regs + 7 * 8]
        mov r9, [rel bsp_regs + 8 * 8]
        mov r10, [rel bsp_regs + 9 * 8]
        mov r11, [rel bsp_regs + 10 * 8]
        mov r12, [rel bsp_regs + 11 * 8]
        mov r13, [rel bsp_regs + 12 * 8]
        mov r14, [rel bsp_regs + 13 * 8]
        mov r15, [rel bsp_regs + 14 * 8]
        ret

;Handlers
extern ih_divide_by_zero
extern ih_double_fault
extern ih_general_protection_fault
extern ih_page_fault

;The actual ISRs
global isr_divide_by_zero
isr_divide_by_zero:
        call save_regs
        call ih_divide_by_zero
        call restore_regs
        iretq

global isr_double_fault
isr_double_fault:
        ;Registers are not saved since this exception is an abort
        pop rdi ;pop the error code (should always be 0)
        call ih_double_fault
        hlt ;halt the core since double faults are an abort

global isr_general_protection_fault
isr_general_protection_fault:
        call save_regs
        pop rdi ;pop the error code
        call ih_general_protection_fault
        hlt
        ;call restore_regs
        ;iretq

global isr_page_fault
isr_page_fault:
        call save_regs
        pop rdi ;pop the error code
        call ih_page_fault
        call restore_regs
        iretq