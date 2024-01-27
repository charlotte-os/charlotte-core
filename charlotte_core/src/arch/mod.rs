#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

use core::fmt::Write;

pub trait Api {
    type Logger: Write;

    fn get_logger() -> Self::Logger;
    fn halt() -> !;
    fn panic() -> !;
    fn inb(port: u16) -> u8;
    fn outb(port: u16, val: u8);
    fn init_bsp();
    fn init_ap();
}

#[cfg(target_arch = "x86_64")]
pub type ArchApi = x86_64::Api;
#[cfg(target_arch = "aarch64")]
pub type ArchApi = aarch64::Api;
#[cfg(target_arch = "riscv64")]
pub type ArchApi = riscv64::Api;
