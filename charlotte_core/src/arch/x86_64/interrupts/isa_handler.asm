// This is the code to handle interrupts, put the data into a struct in memory and call the rust handler
.code64

.text

.global asm_iretq
asm_iretq:
	iretq

.extern save_regs
.extern restore_regs

.extern timer_handler
.global timer_handler
isr_wrapper:
	call save_regs
	call timer_handler
	call restore_regs
	iretq

.global isr_dummy
isr_dummy: // Dummyinterrupt to map to spurious interrupt
	iretq

.global isr_spurious
isr_spurious:
	ret

