//! Interrupts common interfaces, the code in this file is ISA independent
//!

#[cfg(target_arch = "x86_64")]
mod x86_64;

/**
 * Hardware timer mode
 * where supported wheter to run the timer in one shot or recurrent mode.
 */
pub enum HwTimerMode {
    OneShot,
    Recurrent,
}
