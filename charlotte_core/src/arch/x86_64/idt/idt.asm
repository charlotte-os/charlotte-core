bits 64

section .text
global asm_load_idt
asm_load_idt:
        lidt [rdi]
        ret