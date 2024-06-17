#![feature(asm)]

static mut BSP_REGS: [u64; 16] = [0; 16];

pub fn save_regs() {
    unsafe {
        asm!(
        "mov [rip + {0} + 0 * 8], rax",
        "mov [rip + {0} + 1 * 8], rbx",
        "mov [rip + {0} + 2 * 8], rcx",
        "mov [rip + {0} + 3 * 8], rdx",
        "mov [rip + {0} + 4 * 8], rsi",
        "mov [rip + {0} + 5 * 8], rdi",
        "mov [rip + {0} + 6 * 8], rbp",
        "mov [rip + {0} + 7 * 8], r8",
        "mov [rip + {0} + 8 * 8], r9",
        "mov [rip + {0} + 9 * 8], r10",
        "mov [rip + {0} + 10 * 8], r11",
        "mov [rip + {0} + 11 * 8], r12",
        "mov [rip + {0} + 12 * 8], r13",
        "mov [rip + {0} + 13 * 8], r14",
        "mov [rip + {0} + 14 * 8], r15",
        sym BSP_REGS,
        );
    }
}

pub fn restore_regs() {
    unsafe {
        asm!(
        "mov rax, [rip + {0} + 0 * 8]",
        "mov rbx, [rip + {0} + 1 * 8]",
        "mov rcx, [rip + {0} + 2 * 8]",
        "mov rdx, [rip + {0} + 3 * 8]",
        "mov rsi, [rip + {0} + 4 * 8]",
        "mov rdi, [rip + {0} + 5 * 8]",
        "mov rbp, [rip + {0} + 6 * 8]",
        "mov r8, [rip + {0} + 7 * 8]",
        "mov r9, [rip + {0} + 8 * 8]",
        "mov r10, [rip + {0} + 9 * 8]",
        "mov r11, [rip + {0} + 10 * 8]",
        "mov r12, [rip + {0} + 11 * 8]",
        "mov r13, [rip + {0} + 12 * 8]",
        "mov r14, [rip + {0} + 13 * 8]",
        "mov r15, [rip + {0} + 14 * 8]",
        sym BSP_REGS,
        );
    }
}

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

// The actual ISRs
macro_rules! isr {
    ($name:ident, $handler:ident, $save_regs:expr, $restore_regs:expr, $pop_error_code:expr, $halt:expr) => {
        global_asm!(
            concat!(
                stringify!($name),
                ":\n",
                if $save_regs { "call save_regs\n" } else { "" },
                if $pop_error_code { "pop rdi\n" } else { "" },
                "call ", stringify!($handler), "\n",
                if $restore_regs { "call restore_regs\n" } else { "" },
                if $pop_error_code && $restore_regs { "add rsp, 8\n" } else { "" },
                if $halt { "hlt\n" } else { "iretq\n" },
            )
        );
    };
}

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
