bits 64

section .text
global asm_get_vendor_string
asm_get_vendor_string:
        mov r10, rbx ;preserve rbx
        mov eax, 0 ;eax = leaf
        mov ecx, 0 ;ecx = subleaf
        cpuid
        mov [rdi], ebx
        mov [rdi+4], edx
        mov [rdi+8], ecx
        mov rbx, r10 ;restore rbx
        ret