bits 64

section .data
gdtr: 
dw 447 //(64 * 7) - 1
dq 0

section .text
global asm_load_gdt
asm_load_gdt:
        mov [rel gdtr + 2], rdi
        lgdt [rel gdtr]
        ret
global asm_reload_segment_regs
asm_reload_segment_regs:
        mov rax, 1 ;segment descriptor 1 is the kernel code segment
        shl rax, 3
        push rax
        lea rax, [rel reload_cs]
        push rax
        retfq
reload_cs:
        mov ax, 2 ;segment descriptor 2 is the kernel data segment
        shl ax, 3
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov ss, ax
        ret
global asm_load_tss
asm_load_tss:
        mov ax, 5
        shl ax, 3
        ltr ax
        ret
