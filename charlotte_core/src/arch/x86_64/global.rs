//! General asm utilities for x86_64
//! The functions defined here are NOT supposed to be inlined and must be kept in the global_asm! block
//!

use core::arch::global_asm;

global_asm! {
    include_str!("global.asm")
}
