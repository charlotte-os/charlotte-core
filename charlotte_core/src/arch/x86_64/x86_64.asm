.section .text
.global asm_outb
asm_outb:
        mov dx, di
        mov al, sil
        out dx, al
        ret
.global asm_inb
asm_inb:
        mov dx, di
        in al, dx
        ret