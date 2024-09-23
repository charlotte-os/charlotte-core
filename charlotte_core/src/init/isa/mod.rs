use core::cfg_match;

cfg_match! {
    cfg(target_arch = "x86_64") => {
        mod x86_64;
        pub use x86_64::InitApiImpl;
    }
    cfg(target_arch = "riscv64") => {
        mod riscv64;
        pub use riscv64::InitApiImpl;
    }
}
