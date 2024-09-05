use limine::smp::Cpu;
use x86_64::instructions::hlt;

use crate::cpu_control::CpuControl;

pub struct CpuControlX86_64;

impl CpuControl for CpuControlX86_64 {
    fn enable_interrupts() {
        unsafe {
            x86_64::instructions::interrupts::enable();
        }
    }

    fn disable_interrupts() {
        unsafe {
            x86_64::instructions::interrupts::disable();
        }
    }

    fn halt() -> ! {
        loop {
            hlt();
        }
    }
}