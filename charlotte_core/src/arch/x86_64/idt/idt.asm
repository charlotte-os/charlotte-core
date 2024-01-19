.section .text
.global asm_load_idt
asm_load_idt:
        lidt [rip + rdi]
        ret