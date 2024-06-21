// This is the code to handle interrupts, put the data into a struct in memory and call the rust handler
.code64

.text

.global asm_sti
asm_sti:
	sti

.global asm_iretq
asm_iretq:
	iretq

.extern save_regs
.extern restore_regs

.extern handle_int
.global isr_wrapper
isr_wrapper:
	call save_regs
	cld
	call handle_int
	call restore_regs
	iretq

.global isr_dummy
isr_dummy: // Dummyinterrupt to map to spurious interrupt
	iretq

