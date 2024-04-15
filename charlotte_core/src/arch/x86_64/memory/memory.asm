bits 64

section .text
global asm_load_page_map
asm_load_page_map:
    mov cr3, rdi
    ret

global asm_get_cr4
asm_get_cr4:
    mov rax, cr4
    ret

global asm_invalidate_tlb_entry
asm_invalidate_tlb_entry:
    invlpg [rdi]
    ret