mod exceptions;

use crate::logln;

use super::idt::*;


pub fn load_exceptions(idt: &mut Idt) {
    idt.set_gate(0, isr_divide_by_zero, 1 << 3, true, true);
    idt.set_gate(1, isr_debug, 1 << 3, true, false);
    idt.set_gate(2, isr_non_maskable_interrupt, 1 << 3, true, false);
    idt.set_gate(3, isr_breakpoint, 1 << 3, true, false);
    idt.set_gate(4, isr_overflow, 1 << 3, true, false);
    idt.set_gate(5, isr_bound_range_exceeded, 1 << 3, true, false);
    idt.set_gate(6, isr_invalid_opcode, 1 << 3, true, false);
    idt.set_gate(7, isr_device_not_available, 1 << 3, true, false);
    idt.set_gate(8, isr_double_fault, 1 << 3, true, true);
    idt.set_gate(10, isr_invalid_tss, 1 << 3, true, false);
    idt.set_gate(11, isr_segment_not_present, 1 << 3, true, true);
    idt.set_gate(12, isr_stack_segment_fault, 1 << 3, true, false);
    idt.set_gate(13, isr_general_protection_fault, 1 << 3, true, true);
    idt.set_gate(14, isr_page_fault, 1 << 3, true, true);
    idt.set_gate(15, isr_reserved, 1 << 3, true, false);
    idt.set_gate(16, isr_x87_floating_point, 1 << 3, true, false);
    idt.set_gate(17, isr_alignment_check, 1 << 3, true, false);
    idt.set_gate(18, isr_machine_check, 1 << 3, true, false);
    idt.set_gate(19, isr_simd_floating_point, 1 << 3, true, false);
    idt.set_gate(20, isr_virtualization, 1 << 3, true, false);
    idt.set_gate(21, isr_control_protection, 1 << 3, true, false);
    idt.set_gate(28, isr_hypervisor_injection, 1 << 3, true, false);
    idt.set_gate(29, isr_vmm_communication, 1 << 3, true, false);
    idt.set_gate(30, isr_security_exception, 1 << 3, true, false);
}

extern "C" {
    fn isr_divide_by_zero();
    fn isr_debug();
    fn isr_non_maskable_interrupt();
    fn isr_breakpoint();
    fn isr_overflow();
    fn isr_bound_range_exceeded();
    fn isr_invalid_opcode();
    fn isr_device_not_available();
    fn isr_double_fault();
    fn isr_invalid_tss();
    fn isr_stack_segment_fault();
    fn isr_general_protection_fault();
    fn isr_segment_not_present();
    fn isr_page_fault();
    fn isr_reserved();
    fn isr_x87_floating_point();
    fn isr_alignment_check();
    fn isr_machine_check();
    fn isr_simd_floating_point();
    fn isr_virtualization();
    fn isr_control_protection();
    fn isr_hypervisor_injection();
    fn isr_vmm_communication();
    fn isr_security_exception();
}

#[no_mangle]
extern "C" fn ih_double_fault(_error_code: u64) {
    logln!("A double fault has occurred! Panicking!");
    panic!("A double fault has occurred!");
}

#[no_mangle]
extern "C" fn ih_divide_by_zero() {
    logln!("A divide by zero has occurred in kernelspace! Panicking!");
    panic!("A divide by zero has occurred!");
}

#[no_mangle]
extern "C" fn ih_general_protection_fault(error_code: u64, rip: u64) {
    let rip_adjusted = 0xffffffff80000000 - rip;
    if error_code != 0 {
        logln!(
            "A general protection fault has occurred in kernel space with error code {:X}! Panicking!
            this is usually the segment selector that caused the fault. RIP = {:X}",
            error_code, rip_adjusted
        );
    } else {
        logln!(
            "A general protection fault has occurred in kernel space! Panicking! RIP = {:X}",
            rip_adjusted
        );
    }
    panic!("A general protection fault has occurred!");
}

#[no_mangle]
extern "C" fn ih_page_fault(error_code: u64) {
    logln!("A page fault has occurred with error code {:32b}", error_code);
    panic!("A page fault has occurred!");
}

#[no_mangle]
extern "C" fn ih_segment_not_present(error_code: u64) {
    logln!("Segment Not Present Fault: error code {:x}", error_code);
    panic!("Segment Not Present Fault has occurred!");
}

#[no_mangle]
extern "C" fn ih_debug() {
    logln!("Debug Exception Occurred!");
    panic!("Debug Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_non_maskable_interrupt() {
    logln!("Non-maskable Interrupt Occurred!");
    panic!("Non-maskable Interrupt Occurred!");
}

#[no_mangle]
extern "C" fn ih_breakpoint() {
    logln!("Breakpoint Exception Occurred!");
    panic!("Breakpoint Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_overflow() {
    logln!("Overflow Exception Occurred!");
    panic!("Overflow Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_bound_range_exceeded() {
    logln!("Bound Range Exceeded Exception Occurred!");
    panic!("Bound Range Exceeded Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_invalid_opcode() {
    logln!("Invalid Opcode Exception Occurred!");
    panic!("Invalid Opcode Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_device_not_available() {
    logln!("Device Not Available Exception Occurred!");
    panic!("Device Not Available Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_invalid_tss(error_code: u64) {
    logln!("Invalid TSS Exception Occurred! Error code: {:x}", error_code);
    panic!("Invalid TSS Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_stack_segment_fault(error_code: u64) {
    logln!("Stack-Segment Fault Occurred! Error code: {:x}", error_code);
    panic!("Stack-Segment Fault Occurred!");
}

#[no_mangle]
extern "C" fn ih_reserved() {
    logln!("Unexpected Reserved Vector 15 Exception Occurred!");
    panic!("Unexpected Reserved Vector 15 Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_x87_floating_point() {
    logln!("x87 Floating-Point Exception Occurred!");
    panic!("x87 Floating-Point Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_alignment_check(error_code: u64) {
    logln!("Alignment Check Exception Occurred! Error code: {:x}", error_code);
    panic!("Alignment Check Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_machine_check() {
    logln!("Machine Check Exception Occurred! System is halted for safety.");
    panic!("Machine Check Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_simd_floating_point() {
    logln!("SIMD Floating-Point Exception Occurred!");
    panic!("SIMD Floating-Point Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_virtualization() {
    logln!("Virtualization Exception Occurred!");
    panic!("Virtualization Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_control_protection(error_code: u64) {
    logln!("Control Protection Exception Occurred! Error code: {:x}", error_code);
    panic!("Control Protection Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_hypervisor_injection() {
    logln!("Hypervisor Injection Exception Occurred!");
    panic!("Hypervisor Injection Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_vmm_communication(error_code: u64) {
    logln!("VMM Communication Exception Occurred! Error code: {:x}", error_code);
    panic!("VMM Communication Exception Occurred!");
}

#[no_mangle]
extern "C" fn ih_security_exception(error_code: u64) {
    logln!("Security Exception Occurred! Error code: {:x}", error_code);
    panic!("Security Exception Occurred!");
}
