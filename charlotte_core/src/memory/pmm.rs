use crate::{bootinfo, logln};
use core::f32::consts::E;
use core::fmt::Write;

use core::ptr::slice_from_raw_parts_mut;
use core::{
    mem::{align_of, size_of},
    ops::Deref,
};

use limine::memory_map;
use spin::{lazy::Lazy, mutex::Mutex};

static DIRECT_MAP: Lazy<Mutex<PhysicalAddress>> = Lazy::new(|| {
    Mutex::new(
        bootinfo::HHDM_REQUEST
            .get_response()
            .expect("Limine failed to create a direct mapping of physical memory.")
            .offset() as PhysicalAddress,
    )
});


type PhysicalAddress = usize;

pub struct MemoryMap {
    entries: &'static [&'static bootinfo::memory_map::Entry],
}

impl MemoryMap {
    pub fn get() -> MemoryMap {
        MemoryMap {
            entries: bootinfo::MEMORY_MAP_REQUEST
                .get_response()
                .expect("Limine failed to obtain a memory map.")
                .entries(),
        }
    }
    pub fn total_memory(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.entry_type != bootinfo::memory_map::EntryType::BAD_MEMORY)
            .map(|entry| entry.length)
            .sum::<u64>() as usize
    }
    pub fn usable_memory(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .map(|entry| entry.length)
            .sum::<u64>() as usize
    }
    pub fn iter(&self) -> core::slice::Iter<&bootinfo::memory_map::Entry> {
        self.entries.iter()
    }
    pub fn find_best_fit(&self, size: usize) -> Result<&bootinfo::memory_map::Entry, Error> {
        self.entries
            .iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .filter(|entry| entry.length as usize >= size)
            .min_by_key(|entry| entry.length)
            .ok_or(Error::InsufficientContiguousMemoryAvailable)
            .copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    OutOfMemory,
    InsufficientMemoryAvailable,
    InsufficientContiguousMemoryAvailable,
    MemoryOvercommitted,
    PfaRegionArrayFull,
    AllocatedRegionNotFound,
    AddressMisaligned,
    USizeOverflow,
}

const FRAME_SIZE: usize = 4096;

/// A bitmap based physical frame allocator
pub struct PhysicalFrameAllocator {
    region_array: &'static mut [u8]
}

impl PhysicalFrameAllocator {
    fn new() -> PhysicalFrameAllocator {
        let memory_map = MemoryMap::get();
        let total_memory = &memory_map.total_memory();
        // find a region that is large enough to hold the bitmap
        let region = &memory_map.find_best_fit(total_memory / FRAME_SIZE / 8)
            .expect("Failed to find a physical memory region large enough to hold the physical frame allocator bitmap");
        // create the PFA
        let pfa = PhysicalFrameAllocator {
            // Safety: The region is guaranteed to be valid and the length is guaranteed to be large enough to hold the bitmap
            region_array: unsafe { slice_from_raw_parts_mut((DIRECT_MAP.lock().deref() + region.base as usize) as *mut u8, total_memory / FRAME_SIZE as usize).as_mut().unwrap_unchecked()}
        };
        // clear the bitmap to mark all frames as unavailable
        for byte in pfa.region_array.iter_mut() {
            *byte = 1;
        }
        // clear the bits corresponding to available frames
        for entry in MemoryMap::get().iter() {
            if entry.entry_type == bootinfo::memory_map::EntryType::USABLE {
                let start_frame = entry.base as usize / FRAME_SIZE;
                let end_frame = (entry.base + entry.length) as usize / FRAME_SIZE;
                for frame in start_frame..end_frame {
                    pfa.region_array[frame / 8] &= !(1 << (frame % 8));
                }
            }
        }
        // set the bits corresponding to the bitmap as unavailable
        let start_frame = core::ptr::addr_of!(pfa.region_array) as usize / FRAME_SIZE;
        let end_frame = (start_frame + pfa.region_array.len()) / FRAME_SIZE;
        for frame in start_frame..end_frame {
            pfa.region_array[frame / 8] |= 1 << (frame % 8);
        }

        pfa
    }
}
