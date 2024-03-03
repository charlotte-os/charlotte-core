//! # Memory Management Subsystem
//! The memory management subsystem is responsible for managing the direct mapping of physical
//! memory in the kernel's address space, allocating and deallocating physical frames, and managing
//! all virtual address spaces.

pub mod pmm;
pub mod vmm;
