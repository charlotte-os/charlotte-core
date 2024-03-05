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

pub static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> =
    Lazy::new(|| Mutex::new(PhysicalFrameAllocator::new()));

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
    Unavailable(PhysicalAddress),
}

const FRAME_SIZE: usize = 4096;

/// A bitmap based physical frame allocator
pub struct PhysicalFrameAllocator {
    bitmap: &'static mut [u8],
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
            bitmap: unsafe {
                slice_from_raw_parts_mut(
                    (*DIRECT_MAP.lock().deref_mut() + region.base as usize) as *mut u8,
                    total_memory / FRAME_SIZE as usize,
                )
                .as_mut()
                .unwrap_unchecked()
            },
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
    pub fn allocate_contiguous(
        &mut self,
        n_frames: usize,
        alignment: usize,
    ) -> Result<PhysicalAddress, Error> {
        //validate inputs
        if n_frames == 0 {
            return Err(Error::InvalidSize);
        }
        if !alignment.is_power_of_two() {
            return Err(Error::AddressMisaligned);
        }

        // if the requested alignment is less than the frame size, then the alignment is the frame size
        let corrected_alignment = alignment.max(FRAME_SIZE);

        let mut base: PhysicalAddress = 0usize;
        while base < self.bitmap.len() - n_frames * FRAME_SIZE {
            match self.check_region(base, n_frames) {
                RegionAvailability::Available => {
                    for addr in (base..base + n_frames).step_by(FRAME_SIZE) {
                        self.set_by_address(addr);
                    }
                    return Ok(base * FRAME_SIZE);
                }
                RegionAvailability::Unavailable(last_frame) => {
                    // skip to the next properly aligned address after the last frame in the gap using the magic of integer division
                    base = (((last_frame + FRAME_SIZE) / corrected_alignment) + 1)
                        * corrected_alignment;
                    continue;
                }
            }
        }
        Err(Error::InsufficientContiguousMemoryAvailable)
    }

    pub fn deallocate_contiguous(
        &mut self,
        base: PhysicalAddress,
        n_frames: usize,
    ) -> Result<(), Error> {
        // validate inputs
        if n_frames == 0 {
            return Err(Error::InvalidSize);
        }
        if base % FRAME_SIZE != 0 {
            return Err(Error::AddressMisaligned);
        }
        if base / FRAME_SIZE >= self.bitmap.len() {
            return Err(Error::AddressOutOfRange);
        }

        for addr in (base..base + n_frames * FRAME_SIZE).step_by(FRAME_SIZE) {
            self.clear_by_address(addr);
        }
        Ok(())
    }

    fn check_region(&self, base: usize, n_frames: usize) -> RegionAvailability {
        // search the region in reverse order so that if a gap is found
        // the address of the last frame in the gap is returned
        // this is useful for the allocate_contiguous method
        // if a gap is found, the method can continue searching from after the gap
        for i in (0..n_frames).rev() {
            let address = base + i * FRAME_SIZE;
            if self.get_by_address(address) == true {
                return RegionAvailability::Unavailable(address);
            }
        }
        RegionAvailability::Available
    }

    fn index_to_address(&self, byte: usize, bit: usize) -> PhysicalAddress {
        (byte * 8 + bit) * FRAME_SIZE
    }
    fn address_to_index(&self, address: PhysicalAddress) -> (usize, usize) {
        (address / FRAME_SIZE / 8, address / FRAME_SIZE % 8)
    }
    fn get_by_address(&self, address: PhysicalAddress) -> bool {
        let (byte, bit) = self.address_to_index(address);
        self.bitmap[byte] & (1 << bit) != 0
    }
    fn set_by_address(&mut self, address: PhysicalAddress) {
        let (byte, bit) = self.address_to_index(address);
        self.bitmap[byte] |= 1 << bit;
    }
    fn clear_by_address(&mut self, address: PhysicalAddress) {
        let (byte, bit) = self.address_to_index(address);
        self.bitmap[byte] &= !(1 << bit);
    }
}
