.code64

.text
.global asm_halt
asm_halt: // Halt the processor
	cli
	hlt
	jmp asm_halt // Jump to asm_halt in case the processor is not halted

.global asm_inb
asm_inb:
	mov dx, di
	in  al, dx
	ret

.global asm_outb
asm_outb:
	mov dx, di
	mov al, sil
	out dx, al
	ret

.global asm_read_msr
asm_read_msr:
	push   rbp
	mov    rbp, rsp
	mov    [rbp - 4], edi
	mov    [rbp - 16], rsi
	mov    [rbp - 24], rdx
	mov    rax,[rbp - 16]
	mov    [rbp - 40], rax//       # 8-byte Spill
	mov    rax,[rbp - 24]
	mov    [rbp - 32], rax//       # 8-byte Spill
	mov    ecx,[rbp - 4]
	rdmsr
	mov    rcx,[rbp - 40]//       # 8-byte Reload
	mov    esi, eax
	mov    rax,[rbp - 32]//       # 8-byte Reload
	mov    [rcx], esi
	mov    [rax], edx
	pop    rbp
	ret


.global asm_write_msr
asm_write_msr:
	push    rbp
	mov     rbp, rsp
	mov    [rbp - 4], edi
	mov    [rbp - 8], esi
	mov    [rbp - 12], edx
	mov     eax,[rbp - 8]
	mov     edx,[rbp - 12]
	mov     ecx,[rbp - 4]
	wrmsr   // this will have the form MSR[ecx] := edx:eax
	pop     rbp
	ret

.global asm_get_privilege_level
asm_get_privilege_level:
	// this routine takes in 0 params
	mov eax, cs
	and eax, 3
	ret

.global asm_get_vendor_string
asm_get_vendor_string:
	mov r10, rbx //preserve rbx
	mov eax, 0 //eax = leaf
	mov ecx, 0 //ecx = subleaf
	cpuid
	mov[rdi], ebx
	mov[rdi+4], edx
	mov[rdi+8], ecx
	mov rbx, r10 //restore rbx
	ret