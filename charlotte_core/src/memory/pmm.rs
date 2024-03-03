use crate::{bootinfo, logln};
use core::{fmt::Write, ops::DerefMut};

use core::ptr::slice_from_raw_parts_mut;

use spin::{lazy::Lazy, mutex::Mutex};

static DIRECT_MAP: Lazy<Mutex<PhysicalAddress>> = Lazy::new(|| {
    Mutex::new(
        bootinfo::HHDM_REQUEST
            .get_response()
            .expect("Limine failed to create a direct mapping of physical memory.")
            .offset() as PhysicalAddress,
    )
});

static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> = Lazy::new(|| {
    Mutex::new(PhysicalFrameAllocator::new())
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
    AddressMisaligned,
    AddressOutOfRange,
    InvalidSize,
    InvalidAlignment,
}

enum RegionAvailability {
    Available,
    Unavailable(usize),
}

const FRAME_SIZE: usize = 4096;

/// A bitmap based physical frame allocator
pub struct PhysicalFrameAllocator {
    bitmap: &'static mut [u8]
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
            bitmap: unsafe { slice_from_raw_parts_mut((*DIRECT_MAP.lock().deref_mut() + region.base as usize) as *mut u8, total_memory / FRAME_SIZE as usize).as_mut().unwrap_unchecked()}
        };
        // clear the bitmap to mark all frames as unavailable
        for byte in pfa.bitmap.iter_mut() {
            *byte = 1;
        }
        // clear the bits corresponding to available frames
        for entry in MemoryMap::get().iter() {
            if entry.entry_type == bootinfo::memory_map::EntryType::USABLE {
                let start_frame = entry.base as usize / FRAME_SIZE;
                let end_frame = (entry.base + entry.length) as usize / FRAME_SIZE;
                for frame in start_frame..end_frame {
                    pfa.bitmap[frame / 8] &= !(1 << (frame % 8));
                }
            }
        }
        // set the bits corresponding to the bitmap as unavailable
        let start_frame = core::ptr::addr_of!(pfa.bitmap) as usize / FRAME_SIZE;
        let end_frame = (start_frame + pfa.bitmap.len()) / FRAME_SIZE;
        for frame in start_frame..end_frame {
            pfa.bitmap[frame / 8] |= 1 << (frame % 8);
        }

        pfa
    }

    pub fn allocate(&mut self) -> Result<PhysicalAddress, Error> {
        for (byte_index, byte) in self.bitmap.iter_mut().enumerate() {
            if *byte != 0xFF {
                for bit_index in 0..8 {
                    if *byte & (1 << bit_index) == 0 {
                        *byte |= 1 << bit_index;
                        return Ok(byte_index * 8 + bit_index);
                    }
                }
            }
        }
        Err(Error::OutOfMemory)
    }
    pub fn deallocate(&mut self, frame: PhysicalAddress) -> Result<(), Error> {
        if frame % FRAME_SIZE != 0 {
            return Err(Error::AddressMisaligned);
        }
        if frame / FRAME_SIZE >= self.bitmap.len() {
            return Err(Error::AddressOutOfRange);
        }
        self.bitmap[frame / FRAME_SIZE / 8] &= !(1 << (frame / FRAME_SIZE % 8));
        Ok(())
    }
    pub fn allocate_contiguous(&mut self, size: usize, alignment: usize) -> Result<PhysicalAddress, Error> {
        //validate inputs
        if size == 0 {
            return Err(Error::InvalidSize);
        }
        if !Self::is_power_of_two(alignment) {
            return Err(Error::AddressMisaligned);
        }
        
        // if the requested alignment is less than the frame size, then the alignment is the frame size
        let corrected_alignment = if alignment < FRAME_SIZE {
            FRAME_SIZE
        } else {
            alignment
        };
        let corrected_size = ((size / corrected_alignment) + 1) * corrected_alignment;

        let mut base = 0usize;
        while base < self.bitmap.len() - corrected_size / FRAME_SIZE {
            match self.check_region(base, corrected_size) {
                RegionAvailability::Available => {
                    for i in 0..corrected_size / FRAME_SIZE {
                        self.bitmap[base + i] = 0xFF;
                    }
                    if corrected_size % FRAME_SIZE != 0 {
                        self.bitmap[base + corrected_size / FRAME_SIZE] |= 0xFF << (corrected_size % FRAME_SIZE);
                    }
                    return Ok(base * FRAME_SIZE);
                }
                RegionAvailability::Unavailable(last_frame) => {
                    base = last_frame + corrected_alignment / FRAME_SIZE;
                    continue;
                }
            }
        }
        Err(Error::InsufficientContiguousMemoryAvailable)
    }

    pub fn deallocate_contiguous(&mut self, base: PhysicalAddress, size: usize) -> Result<(), Error> {
        if size == 0 {
            return Err(Error::InvalidSize);
        }
        if base % FRAME_SIZE != 0 {
            return Err(Error::AddressMisaligned);
        }
        if base / FRAME_SIZE >= self.bitmap.len() {
            return Err(Error::AddressOutOfRange);
        }
        if size % FRAME_SIZE != 0 {
            return Err(Error::InvalidSize);
        }
        for i in 0..size / FRAME_SIZE {
            self.bitmap[base / FRAME_SIZE + i] = 0;
        }
        if size % FRAME_SIZE != 0 {
            self.bitmap[base / FRAME_SIZE + size / FRAME_SIZE] &= !(0xFF << (size % FRAME_SIZE));
        }

        for addr in base..base + size {
            self.bitmap[addr / FRAME_SIZE] &= !(1 << (addr % FRAME_SIZE));
        }
        Ok(())
    }

    fn check_region(&self, base: usize, length: usize) -> RegionAvailability {
        // search the region in reverse order so that if a gap is found 
        // the address of the last frame in the gap is returned
        // this is useful for the allocate_contiguous method
        // if a gap is found, the method can continue searching from after the gap
        for i in (0..length / FRAME_SIZE).rev() {
            if self.bitmap[base + i] != 0xFF {
                return RegionAvailability::Unavailable(base + i);
            }
        }
        if length % FRAME_SIZE != 0 {
            if self.bitmap[base + length / FRAME_SIZE] & (0xFF << (length % FRAME_SIZE)) != 0xFF << (length % FRAME_SIZE) {
                return RegionAvailability::Unavailable(base + length / FRAME_SIZE);
            }
        }
        RegionAvailability::Available
    }

    fn is_power_of_two(x: usize) -> bool {
        // handle the overflow case
        if x == 0 {
            false
        } else {
            x & (x - 1) == 0
        }
    }

}
