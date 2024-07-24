/// # Memory Allocator
///
/// This module contains the dynamic memory allocator for the kernel. The allocator is responsible for
/// managing the memory used by the kernel. The allocator is used to allocate memory for the kernel
/// and to free memory that is no longer needed.
///
/// The allocator is not thread-safe and instances will thus always need to be placed behind a mutex or a binary semaphore.
use core::ptr::NonNull;

use crate::memory::address::VirtualAddress;

struct Allocation {
    base: VirtualAddress,
    size: usize,
    next: Option<NonNull<Allocation>>,
}

struct Allocator<const base: VirtualAddress, const size: usize> {
    large_allocations: Option<NonNull<Allocation>>,
    small_allocations: Option<NonNull<Allocation>>,
}
