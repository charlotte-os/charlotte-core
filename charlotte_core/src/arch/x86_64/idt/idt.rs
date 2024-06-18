use core::arch::global_asm;

global_asm! {
    "
.code64

.text
.global asm_load_idt
asm_load_idt:
    lidt [rdi]
    ret
"
}
