use limine::smp::Cpu;
use x86_64::instructions::hlt;

use crate::cpu_control::CpuControl;

pub struct CpuControlX86_64;

impl CpuControl for CpuControlX86_64 {
    #[inline]
    fn enable_interrupts() {
        unsafe {
            x86_64::instructions::interrupts::enable();
        }
    }
    #[inline]
    fn disable_interrupts() {
        unsafe {
            x86_64::instructions::interrupts::disable();
        }
    }
    #[inline]
    fn halt() -> ! {
        loop {
            hlt();
        }
    }
}
