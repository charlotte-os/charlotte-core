//! # Memory Management Subsystem
//! The memory management subsystem is responsible for managing the direct mapping of physical
//! memory in the kernel's address space, allocating and deallocating physical frames, and managing
//! all virtual address spaces.

use aarch64_cpu::registers::Readable;
use spin::lazy::Lazy;

/// The number of significant bits in a physical address on the current CPU.
pub static PADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
    use aarch64_cpu::registers::ID_AA64MMFR0_EL1;
    use aarch64_cpu::registers::ID_AA64MMFR0_EL1::PARange::*;

    match ID_AA64MMFR0_EL1
        .read_as_enum::<ID_AA64MMFR0_EL1::PARange::Value>(ID_AA64MMFR0_EL1::PARange)
    {
        Some(Value::Bits_32) => 32,
        Some(Value::Bits_36) => 36,
        Some(Value::Bits_40) => 40,
        Some(Value::Bits_42) => 42,
        Some(Value::Bits_44) => 44,
        Some(Value::Bits_48) => 48,
        Some(Value::Bits_52) => 52,
        _ => panic!("CPU is broken"),
    }
});

/// The number of significant bits in a virtual address on the current CPU.
pub static VADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
    use aarch64_cpu::registers::ID_AA64MMFR2_EL1;

    match ID_AA64MMFR2_EL1.read(ID_AA64MMFR2_EL1::VARange) {
        0 => 48,
        1 => 52,
        _ => panic!("CPU is broken"),
    }
});
