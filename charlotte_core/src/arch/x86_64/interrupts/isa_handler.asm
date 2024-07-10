// This is the code to handle interrupts, put the data into a struct in memory and call the rust handler
.code64

.text

.global isr_dummy
isr_dummy: // Dummyinterrupt to map to spurious interrupt
	iretq
