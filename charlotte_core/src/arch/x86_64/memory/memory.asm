bits 64

section .text
global asm_load_page_map
asm_load_page_map:
    mov cr3, rdi
    ret
    