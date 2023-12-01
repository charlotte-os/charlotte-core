#[cfg(target_arch="x86_64")]
pub mod x86_64;
#[cfg(target_arch="aarch64")]
pub mod aarch64;
#[cfg(target_arch="riscv64")]
pub mod riscv64;

use core::fmt::Write;

pub trait Arch {
        type Logger: Write;

        fn halt() -> !;
        fn get_logger() -> Self::Logger;
}

#[cfg(target_arch="x86_64")]
pub type ArchApi = x86_64::ArchApi;
#[cfg(target_arch="aarch64")]
pub type ArchApi = aarch64::ArchApi;
#[cfg(target_arch="riscv64")]
pub type ArchApi = riscv64::ArchApi;