#![feature(asm, global_asm)]

static mut BSP_REGS: [u64; 16] = [0; 16];

// ISR macro definition
macro_rules! isr {
    ($name:ident, $handler:ident, $save_regs:expr, $restore_regs:expr, $pop_error_code:expr, $halt:expr) => {
        global_asm!(
            concat!(
                stringify!($name),
                ":\n",
                if $save_regs {
                    "mov [rip + ", stringify!(BSP_REGS), " + 0 * 8], rax\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 1 * 8], rbx\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 2 * 8], rcx\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 3 * 8], rdx\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 4 * 8], rsi\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 5 * 8], rdi\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 6 * 8], rbp\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 7 * 8], r8\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 8 * 8], r9\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 9 * 8], r10\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 10 * 8], r11\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 11 * 8], r12\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 12 * 8], r13\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 13 * 8], r14\n",
                    "mov [rip + ", stringify!(BSP_REGS), " + 14 * 8], r15\n",
                } else { "" },
                if $pop_error_code { "pop rdi\n" } else { "" },
                "call ", stringify!($handler), "\n",
                if $restore_regs {
                    "mov rax, [rip + ", stringify!(BSP_REGS), " + 0 * 8]\n",
                    "mov rbx, [rip + ", stringify!(BSP_REGS), " + 1 * 8]\n",
                    "mov rcx, [rip + ", stringify!(BSP_REGS), " + 2 * 8]\n",
                    "mov rdx, [rip + ", stringify!(BSP_REGS), " + 3 * 8]\n",
                    "mov rsi, [rip + ", stringify!(BSP_REGS), " + 4 * 8]\n",
                    "mov rdi, [rip + ", stringify!(BSP_REGS), " + 5 * 8]\n",
                    "mov rbp, [rip + ", stringify!(BSP_REGS), " + 6 * 8]\n",
                    "mov r8, [rip + ", stringify!(BSP_REGS), " + 7 * 8]\n",
                    "mov r9, [rip + ", stringify!(BSP_REGS), " + 8 * 8]\n",
                    "mov r10, [rip + ", stringify!(BSP_REGS), " + 9 * 8]\n",
                    "mov r11, [rip + ", stringify!(BSP_REGS), " + 10 * 8]\n",
                    "mov r12, [rip + ", stringify!(BSP_REGS), " + 11 * 8]\n",
                    "mov r13, [rip + ", stringify!(BSP_REGS), " + 12 * 8]\n",
                    "mov r14, [rip + ", stringify!(BSP_REGS), " + 13 * 8]\n",
                    "mov r15, [rip + ", stringify!(BSP_REGS), " + 14 * 8]\n",
                } else { "" },
                if $pop_error_code && $restore_regs { "add rsp, 8\n" } else { "" },
                if $halt { "hlt\n" } else { "iretq\n" },
            )
        );
    };
}

// Declare all ISRs
isr!(isr_divide_by_zero, ih_divide_by_zero, true, true, false, false);
isr!(isr_double_fault, ih_double_fault, false, false, true, true);
isr!(isr_general_protection_fault, ih_general_protection_fault, true, false, true, true);
isr!(isr_page_fault, ih_page_fault, true, true, true, false);
isr!(isr_segment_not_present, ih_segment_not_present, true, true, true, false);
isr!(isr_debug, ih_debug, true, true, false, false);
isr!(isr_non_maskable_interrupt, ih_non_maskable_interrupt, true, true, false, false);
isr!(isr_breakpoint, ih_breakpoint, true, true, false, false);
isr!(isr_overflow, ih_overflow, true, true, false, false);
isr!(isr_bound_range_exceeded, ih_bound_range_exceeded, true, true, false, false);
isr!(isr_invalid_opcode, ih_invalid_opcode, true, true, false, false);
isr!(isr_device_not_available, ih_device_not_available, true, true, false, false);
isr!(isr_invalid_tss, ih_invalid_tss, true, true, true, false);
isr!(isr_stack_segment_fault, ih_stack_segment_fault, true, true, true, false);
isr!(isr_reserved, ih_reserved, true, true, false, false);
isr!(isr_x87_floating_point, ih_x87_floating_point, true, true, false, false);
isr!(isr_alignment_check, ih_alignment_check, true, true, true, false);
isr!(isr_machine_check, ih_machine_check, false, false, false, true);
isr!(isr_simd_floating_point, ih_simd_floating_point, true, true, false, false);
isr!(isr_virtualization, ih_virtualization, true, true, false, false);
isr!(isr_control_protection, ih_control_protection, true, true, true, false);
isr!(isr_hypervisor_injection, ih_hypervisor_injection, true, true, false, false);
isr!(isr_vmm_communication, ih_vmm_communication, true, true, true, false);
isr!(isr_security_exception, ih_security_exception, true, true, true, false);

// Handlers
extern "C" {
    fn ih_divide_by_zero();
    fn ih_double_fault();
    fn ih_general_protection_fault();
    fn ih_page_fault();
    fn ih_segment_not_present();
    fn ih_debug();
    fn ih_non_maskable_interrupt();
    fn ih_breakpoint();
    fn ih_overflow();
    fn ih_bound_range_exceeded();
    fn ih_invalid_opcode();
    fn ih_device_not_available();
    fn ih_invalid_tss();
    fn ih_stack_segment_fault();
    fn ih_reserved();
    fn ih_x87_floating_point();
    fn ih_alignment_check();
    fn ih_machine_check();
    fn ih_simd_floating_point();
    fn ih_virtualization();
    fn ih_control_protection();
    fn ih_hypervisor_injection();
    fn ih_vmm_communication();
    fn ih_security_exception();
}
