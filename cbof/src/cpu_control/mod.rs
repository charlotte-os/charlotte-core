mod isa;

pub trait CpuControl {
    fn enable_interrupts();
    fn disable_interrupts();
    fn halt() -> !;
}

#[cfg(target_arch = "x86_64")]
pub type CpuControlIfce = isa::x86_64::CpuControlX86_64;
