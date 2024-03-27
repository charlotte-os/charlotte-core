# CharlotteOS
# Ahmed Hussein (amhussein4@gmail.com)
# 2/11/2024

# A set of primitive getters and setters that access the active 
# hart's (core) control and status registers to get information 
# about its current operational status and control it. 

# Unless otherwise stated, all functions that return values 
# return them in register a0 and all functions that take 
# arguments take them in register a0. All functions take at 
# most one argument and return at most one argument. 

# Get hart (core) ID
.global hart_id
hart_id:
	csrr a0, mhartid
	ret

# Status control
# Read and write machine status register
.global read_machine_status
read_machine_status:
	csrr a0, mstatus
	ret

.global write_machine_status
write_machine_status:
	csrw mstatus, a0
	ret

# Read and write supervisor status register
.global read_supervisor_status
read_supervisor_status:
	csrr a0, sstatus
	ret

.global write_supervisor_status
write_supervisor_status:
	csrw sstatus, a0
	ret

# Global interrupt control
# Enable and disable interrupts globally in supervisor mode
.global globally_enable_interrupts
globally_enable_interrupts:
	csrr a0, sstatus
	ori a0, a0, 0x02
	csrw sstatus, a0
	ret

.global globally_disable_interrupts
globally_disable_interrupts:
	csrr a0, sstatus
	andi a0, a0, 0xFFD
	csrw sstatus, a0
	ret

# Check if supervisor interrupts are globally enabled
.global interrupts_globally_enabled
interrupts_globally_enabled:
	csrr a0, sstatus
	andi a0, a0, 0x02
	srli a0, a0, 1
	ret

# Trap enable control
# Get and set enabled machine traps
.global get_machine_enabled_traps
get_machine_enabled_traps:
	csrr a0, mie
	ret

.global set_machine_enabled_traps
set_machine_enabled_traps:
	csrw mie, a0
	ret

# Get and set enabled supervisor traps
.global get_supervisor_enabled_traps
get_supervisor_enabled_traps:
	csrr a0, sie
	ret

.global set_supervisor_enabled_traps
set_supervisor_enabled_traps:
	csrw sie, a0
	ret

# Pending Traps
# Get and set supervisor pending traps
.global get_pending_traps
get_pending_traps:
	csrr a0, sip
	ret

.global set_pending_traps
set_pending_traps:
	csrw sip, a0
	ret

# Trap Delegation
# Get and set machine delegated interrupts
.global get_machine_delegated_interrupts
get_machine_delegated_interrupts:
	csrr a0, mideleg
	ret

.global set_machine_delegated_interrupts
set_machine_delegated_interrupts:
	csrw mideleg, a0
	ret

# Get and set machine delegated exceptions
.global get_machine_delegated_exceptions
get_machine_delegated_exceptions:
	csrr a0, medeleg
	ret

.global set_machine_delegated_exceptions
set_machine_delegated_exceptions:
	csrw medeleg, a0
	ret

# Interrupt vector base address control
# Lower two bits of the mtvec and stvec registers give the trap 
# vector mode (direct or vectorized)
# Get and set machine and supervisor trap vector location
.global set_machine_trap_vector_location
set_machine_trap_vector_location:
	csrw mtvec, a0
	ret

.global get_supervisor_trap_vector_location
get_supervisor_trap_vector_location:
	csrr a0, stvec
	ret

.global set_supervisor_trap_vector_location
set_supervisor_trap_vector_location:
	csrw stvec, a0
	ret

# Trap return address
# Get and set machine trap return location
.global get_machine_trap_return_location
get_machine_trap_return_location:
	csrr a0, mepc
	ret

.global set_machine_trap_return_location
set_machine_trap_return_location:
	csrw mepc, a0
	ret

# Get and set supervisor trap return location
.global get_supervisor_trap_return_location
get_supervisor_trap_return_location:
	csrr a0, sepc
	ret

.global set_supervisor_trap_return_location
set_supervisor_trap_return_location:
	csrw sepc, a0
	ret

# Trap cause, value and scratch access
.global get_supervisor_trap_cause
get_supervisor_trap_cause:
	csrr a0, scause
	ret

.global get_supervisor_trap_value
get_supervisor_trap_value:
	csrr a0, stval
	ret

.global write_machine_scratch
write_machine_scratch:
	csrw mscratch, a0
	ret

# Counters, monitors and their control
.global get_machine_enabled_counters
get_machine_enabled_counters:
	csrr a0, mcounteren
	ret

.global set_machine_enabled_counters
set_machine_enabled_counters:
	csrw mcounteren, a0
	ret

# Thread and stack access
# Get and set thread pointer
.global get_thread
get_thread:
	mv a0, tp
	ret

.global set_thread
set_thread:
	mv tp, a0
	ret

# Get and set stack pointer
.global get_stack_location
get_stack_location:
	mv a0, sp
	ret

.global set_stack_location
set_stack_location:
	mv sp, a0
	ret

# Timer access
.global get_time
get_time:
	mv a0, mtime
	ret

# Memory paging
# Get and set supervisor-mode Page table base pointer
.global get_page_table_location
get_page_table_location:
	csrr a0, satp
	ret

.global set_page_table_location
set_page_table_location:
	csrw satp, a0
	ret

.global flush_tlb
flush_tlb:
	sfence.vma zero, zero
	ret

# Physical Memory Protection
.global set_pmp_configuration
set_pmp_configuration:
	csrw pmpcfg0, a0
	ret

.global set_pmp_address
set_pmp_address:
	csrw pmpaddr0, a0
	ret
