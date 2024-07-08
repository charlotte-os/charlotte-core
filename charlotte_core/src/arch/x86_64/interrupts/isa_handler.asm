// This is the code to handle interrupts, put the data into a struct in memory and call the rust handler
.code64

.text

.global asm_iretq
asm_iretq:
	iretq

.global isr_dummy
isr_dummy: // Dummyinterrupt to map to spurious interrupt
	iretq

.global isr_spurious
isr_spurious:
	ret

