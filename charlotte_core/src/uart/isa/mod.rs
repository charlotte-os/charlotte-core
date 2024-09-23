//! # UART ISA-specific Components
//! Automatically pulls in the correct architecture-specific implementation details

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use x86_64::*;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv64;
        pub use riscv64::*;
    } else {
        compile_error!("Unsupported target architecture");
    }
}
