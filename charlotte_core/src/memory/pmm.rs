use crate::bootinfo;
use crate::memory::address::PhysicalAddress;

use core::slice::from_raw_parts_mut;

use spin::{lazy::Lazy, mutex::Mutex};

static DIRECT_MAP: Lazy<PhysicalAddress> = Lazy::new(|| {
    PhysicalAddress::new(
        bootinfo::HHDM_REQUEST
            .get_response()
            .expect("Limine failed to create a direct mapping of physical memory.")
            .offset() as usize,
    )
});

pub static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> =
    Lazy::new(|| Mutex::new(PhysicalFrameAllocator::new()));

pub struct MemoryMap {
    entries: &'static [&'static bootinfo::memory_map::Entry],
}

#[allow(unused)]
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
        self.entries.iter().map(|entry| entry.length).sum::<u64>() as usize
    }

    pub fn usable_memory(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .map(|entry| entry.length)
            .sum::<u64>() as usize
    }

    fn highest_address(&self) -> usize {
        self.entries
            .iter()
            .map(|entry| entry.base + entry.length)
            .max()
            .unwrap_or(0) as usize
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
#[allow(unused)]
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
        let total_memory = memory_map.highest_address();
        let bitmap_len = (total_memory / FRAME_SIZE).div_ceil(u8::BITS as usize);
        // find a region that is large enough to hold the bitmap
        let region = memory_map.find_best_fit(bitmap_len)
            .expect("Failed to find a physical memory region large enough to hold the physical frame allocator bitmap");

        // Initialize bitmap and create PFA
        let bitmap_addr = (*DIRECT_MAP + region.base as usize).bits() as *mut u8;
        let bitmap = unsafe {
            // clear the bitmap to mark all frames as unavailable
            bitmap_addr.write_bytes(0xff, bitmap_len);
            from_raw_parts_mut(bitmap_addr, bitmap_len)
        };

        let mut pfa = PhysicalFrameAllocator { bitmap };

        // clear the bits corresponding to available frames
        for entry in MemoryMap::get().iter() {
            if entry.entry_type == bootinfo::memory_map::EntryType::USABLE {
                let start = PhysicalAddress::new(entry.base as usize);
                let n_frames = entry.length as usize / FRAME_SIZE;
                for addr in start.iter_frames(n_frames) {
                    pfa.clear_by_address(addr);
                }
            } else {
                // for unusable regions (like BAD_MEMORY), ensure the bits are set to 1 (unavailable)
                let start = PhysicalAddress::new(entry.base as usize);
                let n_frames = entry.length as usize / FRAME_SIZE;
                for addr in start.iter_frames(n_frames) {
                    pfa.set_by_address(addr);
                }
            }
        }

        // set the bits corresponding to the bitmap as unavailable
        let bitmap_start = PhysicalAddress::new(region.base as usize);
        let bitmap_frames = (region.length as usize).div_ceil(FRAME_SIZE);
        for addr in bitmap_start.iter_frames(bitmap_frames) {
            pfa.set_by_address(addr);
        }

        pfa
    }

    #[inline]
    const fn frame_capacity(&self) -> usize {
        self.bitmap.len() * 8
    }

    pub fn allocate(&mut self) -> Result<PhysicalAddress, Error> {
        for (byte_index, byte) in self.bitmap.iter_mut().enumerate() {
            let bit_index = byte.trailing_ones() as usize;
            if bit_index < 8 {
                *byte |= 1 << bit_index;
                return Ok(self.index_to_address(byte_index, bit_index));
            }
        }
        Err(Error::OutOfMemory)
    }

    pub fn deallocate(&mut self, frame: PhysicalAddress) -> Result<(), Error> {
        if !frame.is_page_aligned() {
            return Err(Error::AddressMisaligned);
        }
        if frame.pfn() >= self.frame_capacity() {
            return Err(Error::AddressOutOfRange);
        }
        self.clear_by_address(frame);
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

        let mut base = PhysicalAddress::new(0);
        while base.pfn() + n_frames < self.frame_capacity() {
            match self.check_region(base, n_frames) {
                RegionAvailability::Available => {
                    for addr in base.iter_frames(n_frames) {
                        self.set_by_address(addr);
                    }
                    return Ok(base);
                }
                RegionAvailability::Unavailable(last_frame) => {
                    // skip to the next properly aligned address after the last frame in the gap
                    base = PhysicalAddress::new(
                        (last_frame.bits() + FRAME_SIZE).next_multiple_of(corrected_alignment),
                    );
                }
            }
        }
        Err(Error::InsufficientContiguousMemoryAvailable)
    }

    #[allow(unused)]
    pub fn deallocate_contiguous(
        &mut self,
        base: PhysicalAddress,
        n_frames: usize,
    ) -> Result<(), Error> {
        // validate inputs
        if n_frames == 0 {
            return Err(Error::InvalidSize);
        }
        if !base.is_page_aligned() {
            return Err(Error::AddressMisaligned);
        }
        if base.pfn() >= self.frame_capacity() || base.pfn() + n_frames >= self.frame_capacity() {
            return Err(Error::AddressOutOfRange);
        }

        for addr in base.iter_frames(n_frames) {
            self.clear_by_address(addr);
        }
        Ok(())
    }

    fn check_region(&self, base: PhysicalAddress, n_frames: usize) -> RegionAvailability {
        // search the region in reverse order so that if a gap is found
        // the address of the last frame in the gap is returned
        // this is useful for the allocate_contiguous method
        // if a gap is found, the method can continue searching from after the gap
        for address in base.iter_frames(n_frames).rev() {
            if self.get_by_address(address) {
                return RegionAvailability::Unavailable(address);
            }
        }
        RegionAvailability::Available
    }

    fn index_to_address(&self, byte: usize, bit: usize) -> PhysicalAddress {
        PhysicalAddress::from_pfn(byte * 8 + bit)
    }

    fn address_to_index(&self, address: PhysicalAddress) -> (usize, usize) {
        (address.pfn() / 8, address.pfn() % 8)
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
