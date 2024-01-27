bits 64

section .text
global asm_cpuid
asm_cpuid:
        mov r10, rbx ;preserve rbx
        mov eax, esi ;eax = leaf
        mov ecx, edx ;ecx = subleaf
        cpuid
        mov [rdi], rax
        mov [rdi+4], rbx
        mov [rdi+8], rcx
        mov [rdi+12], rdx
        mov rbx, r10 ;restore rbx
        ret
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