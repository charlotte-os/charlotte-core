use core::{slice, usize};

pub struct RingBuffer<'data, T: Sized> {
    data: &'data mut [T],
    cap: usize,
    read_ptr: usize,
    write_ptr: usize,
}

#[inline(always)]
pub fn is_aligned(addr: usize, align: usize) -> bool {
    if align == 0 {
        false
    } else {
        addr & (align - 1) == 0
    }
}

impl<'data, T> RingBuffer<'data, T> {
    /// Create a new ring buffer from a slice of static memory
    /// ## Safety, this function will have UB if the following isn't respected:
    ///     * The memory at data must be valid for read/write for `cap * mem::sizeof::<T>()`
    ///     * The memory is contigously allocated
    ///     * data is non null and aligned
    pub fn new_in_static_memory(data: *mut T, cap: usize) -> Self {
        if is_aligned(data as usize, 0x1000) {
            panic!("Tried to make new ring buffer out of unaligned address");
        }
        RingBuffer {
            data: unsafe { slice::from_raw_parts_mut(data, cap) },
            cap,
            read_ptr: 0,
            write_ptr: 0,
        }
    }
}

impl<'data, T> Drop for RingBuffer<'data, T> {
    fn drop(&mut self) {}
}
