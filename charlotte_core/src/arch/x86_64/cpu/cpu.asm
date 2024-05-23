bits 64

section .text
global asm_halt
asm_halt:
        cli
        hlt
        jmp asm_halt

global asm_inb
asm_inb:
        mov dx, di
        in al, dx
        ret

global asm_outb
asm_outb:
        mov dx, di
        mov al, sil
        out dx, al
        ret

global asm_read_msr
asm_read_msr:
        push    rbp
        mov     rbp, rsp
        mov     dword [rbp - 4], edi
        mov     qword [rbp - 16], rsi
        mov     qword [rbp - 24], rdx
        mov     rax, qword [rbp - 16]
        mov     qword [rbp - 40], rax;       # 8-byte Spill
        mov     rax, qword [rbp - 24]
        mov     qword [rbp - 32], rax;       # 8-byte Spill
        mov     ecx, dword [rbp - 4]
        rdmsr
        mov     rcx, qword [rbp - 40];       # 8-byte Reload
        mov     esi, eax
        mov     rax, qword [rbp - 32];       # 8-byte Reload
        mov     dword [rcx], esi
        mov     dword [rax], edx
        pop     rbp
        ret


global asm_write_msr
asm_write_msr:
        push    rbp
        mov     rbp, rsp
        mov     dword  [rbp - 4], edi
        mov     dword  [rbp - 8], esi
        mov     dword  [rbp - 12], edx
        mov     eax, dword  [rbp - 8]
        mov     edx, dword  [rbp - 12]
        mov     ecx, dword  [rbp - 4]
        wrmsr   ; this will have the form MSR[ecx] := edx:eax 
        pop     rbp
        ret

global asm_get_privilege_level
asm_get_privilege_level:
        ; this routine takes in 0 params
        mov eax, cs
        and eax, 3
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
