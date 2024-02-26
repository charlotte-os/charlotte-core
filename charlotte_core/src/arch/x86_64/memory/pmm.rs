use crate::bootinfo;

use core::{cmp::PartialOrd, mem::size_of};

use spin::{lazy::Lazy, mutex::Mutex};

static DIRECT_MAP: Lazy<Mutex<PhysicalAddress>> = Lazy::new(|| {
    Mutex::new(bootinfo::HHDM_REQUEST.get_response()
        .expect("Limine failed to create a direct mapping of physical memory.")
        .offset()
    )
});

type PhysicalAddress = u64;

struct MemoryMap {
    entries: &'static [&'static bootinfo::memory_map::Entry]
}

impl MemoryMap {
    pub fn get() -> MemoryMap {
        MemoryMap {
            entries: bootinfo::MEMORY_MAP_REQUEST.get_response()
                .expect("Limine failed to obtain a memory map.")
                .entries()
        }
    }
    pub fn total_memory(&self) -> usize {
        self.entries.iter()
            .filter(|entry| entry.entry_type != bootinfo::memory_map::EntryType::BAD_MEMORY)
            .map(|entry| entry.length)
            .sum::<u64>() as usize
    }
    pub fn usable_memory(&self) -> usize {
        self.entries.iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .map(|entry| entry.length)
            .sum::<u64>() as usize
    }
    pub fn iter(&self) -> core::slice::Iter<bootinfo::memory_map::Entry> {
        self.entries.iter()
    }
    pub fn find_best_fit(&self, size: usize) -> &mut bootinfo::memory_map::Entry {
        self.entries.iter_mut()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .filter(|entry| entry.length as usize >= size)
            .min_by_key(|entry| entry.length)
            .expect("The memory map is empty or there is no usable memory.")
    }
}

enum Error {
    OutOfMemory,
    InsufficientMemoryAvailable,
    InsufficientContiguousMemoryAvailable,
    MemoryOvercommitted,
    PfaRegionArrayFull,
    AllocatedRegionNotFound,
    AddressMisaligned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMemoryRegion {
    base: PhysicalAddress,
    size: usize,
    is_available: bool,
}

impl PartialOrd for PhysicalMemoryRegion {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.base.partial_cmp(&other.base)
    }
}

impl PhysicalMemoryRegion {
    fn new(base: PhysicalAddress, size: usize, is_available: bool) -> Self {
        PhysicalMemoryRegion {
            base,
            size,
            is_available,
        }
    }
}

impl Ord for PhysicalMemoryRegion {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.base.cmp(&other.base)
    }
}

const FRAME_SIZE: usize = 4096;


pub struct PhysicalFrameAllocator {
    region_array: &'static mut [Option<PhysicalMemoryRegion>],
    len: usize, //number of used elements in the region array
}

// in frames
const ESTIMATED_AVG_REGION_SIZE: usize = 128;

impl PhysicalFrameAllocator {
    fn new() -> Self {
        let memory_map = MemoryMap::get();
        // Use the same amount of memory as a bitmap would for the region array
        let region_array_size = ((memory_map.total_memory() / FRAME_SIZE) * size_of::<PhysicalMemoryRegion>()) / ESTIMATED_AVG_REGION_SIZE;
        let mut best_fit_entry = memory_map.find_best_fit(region_array_size);
        let pfa_base = best_fit_entry.base + best_fit_entry.length - region_array_size as u64;
        let pfa = PhysicalFrameAllocator {
            region_array: unsafe { core::slice::from_raw_parts_mut((DIRECT_MAP.lock() + pfa_base) as *mut Option<PhysicalMemoryRegion>, region_array_size) },
            len: 0,
        };
        memory_map.iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .for_each(|entry| pfa.add_region(PhysicalMemoryRegion {
                base: entry.base,
                size: entry.length as usize,
                is_available: true,
            }));
        // Sort the region array so that we can use binary search to find regions
        pfa.region_array.sort_unstable();
        // merge adjacent regions
        pfa.merge_adjacent_regions();

        pfa
    }

    fn add_region(&mut self, region: PhysicalMemoryRegion) {
        self.region_array[self.len] = Some(region);
        self.len += 1;
    }

    /// Combines all contiguous regions of the same type into singular regions
    fn compact_regions(&mut self) {
        self.region_array.sort_unstable();
        for i in 0..self.len {
            if let Some(region) = &mut self.region_array[i] {
                for j in (i + 1)..self.len {
                    if let Some(next_region) = self.region_array[j] {
                        if region.base + region.size as u64 == next_region.base {
                            region.size += next_region.size;
                            self.region_array[j] = None;
                        }
                    }
                }
            }
        }
        self.region_array.sort_unstable();
    }

    /// Returns the index of the smallest region
    fn smallest_region_index(&self) -> Result<usize, Error> {
        self.region_array.iter()
            .enumerate()
            .filter(|(_, region)| {
                if region.is_some() {
                    region.as_ref().unwrap().is_available
                } else {
                    false
                }
            })
            .min_by_key(|(_, region)| region.as_ref().unwrap().size)
            .ok_or(Error::OutOfMemory)
    }

    fn largest_region_index(&self) -> Result<usize, Error> {
        self.region_array.iter()
            .enumerate()
            .filter(|(_, region)| {
                if region.is_some() {
                    region.as_ref().unwrap().is_available
                } else {
                    false
                }
            })
            .max_by_key(|(_, region)| region.as_ref().unwrap().size)
            .ok_or(Error::OutOfMemory)
    }

    pub fn available_memory(&self) -> usize {
        self.region_array.iter()
            .filter(|region| {
                if region.is_some() {
                    region.as_ref().unwrap().is_available
                } else {
                    false
                }
            })
            .map(|region| region.as_ref().unwrap().size)
            .sum()
    }

    fn best_fit_region_index(&self, size: usize) -> Result<usize, Error> {
        if self.available_memory() < size {
            if self.available_memory() == 0 {
                Err(Error::OutOfMemory)
            } else {
                Err(Error::InsufficientMemoryAvailable)
            }
        } else {
            self.region_array.iter()
                .enumerate()
                .filter(|(_, region)| {
                    if region.is_some() {
                        region.as_ref().unwrap().is_available
                    } else {
                        false
                    }
                })
                .filter(|(_, region)| region.as_ref().unwrap().size >= size)
                .min_by_key(|(_, region)| region.as_ref().unwrap().size)
                .ok_or(Error::InsufficientContiguousMemoryAvailable)
        }
    }

    pub fn allocate_frames(&mut self, count: usize) -> Result<PhysicalMemoryRegion, Error> {
        let alloc_region_index = self.best_fit_region_index(count * FRAME_SIZE)?;
        // Safety: The best_fit_region_index function ensures that the region is available
        let mut alloc_region = unsafe { self.region_array[alloc_region_index].unwrap_unchecked() };
        alloc_region.size -= count * FRAME_SIZE;
        let new_region = PhysicalMemoryRegion::new(alloc_region.base + alloc_region.size - count * FRAME_SIZE, count * FRAME_SIZE, false);
        self.add_region(new_region);

        Ok(new_region)
    }
}

struct PhysicalMemoryManager {
    pfa: PhysicalFrameAllocator,
    unallocated_frames_committed: usize,
}