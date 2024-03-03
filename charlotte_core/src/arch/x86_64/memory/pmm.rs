use crate::bootinfo;

use core::{borrow::{Borrow, BorrowMut}, cmp::PartialOrd, mem::size_of, ops::Deref};

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

struct MemoryMap {
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
    pub fn find_best_fit(&self, size: usize) -> &bootinfo::memory_map::Entry {
        self.entries
            .iter()
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

impl PhysicalMemoryRegion {
    fn contains(&self, other: &Self) -> bool {
        self.base <= other.base && self.base + self.size >= other.base + other.size
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
        let region_array_size = ((memory_map.total_memory() / FRAME_SIZE)
            * size_of::<PhysicalMemoryRegion>())
            / ESTIMATED_AVG_REGION_SIZE;
        let best_fit_entry = memory_map.find_best_fit(region_array_size);
        let pfa_base = (best_fit_entry.base + best_fit_entry.length - region_array_size as u64) as usize;
        let mut pfa = PhysicalFrameAllocator {
            region_array: unsafe {
                core::slice::from_raw_parts_mut(
                    (DIRECT_MAP.lock().deref() + pfa_base) as *mut Option<PhysicalMemoryRegion>,
                    region_array_size,
                )
            },
            len: 0,
        };
        memory_map
            .iter()
            .filter(|entry| entry.entry_type == bootinfo::memory_map::EntryType::USABLE)
            .for_each(|entry| {
                pfa.add_region(PhysicalMemoryRegion {
                    base: entry.base as usize,
                    size: entry.length as usize,
                    is_available: true,
                })
            });
        // merge adjacent regions
        pfa.combine_regions();
        // Sort the region array so that we can use binary search to find regions
        pfa.region_array.sort_unstable();

        pfa
    }

    fn add_region(&mut self, region: PhysicalMemoryRegion) {
        self.region_array[self.len] = Some(region);
        self.len += 1;
    }

    /// Combines all contiguous regions of the same type into singular regions
    fn combine_regions(&mut self) {
        let mut i = 0usize;
        while self.region_array[i].is_some() {
            let mut j = i + 1;
            while self.region_array[j].is_some() {
                // if region_array[i] and region_array[j] are contiguous and have the same availability
                // then increase the size of region_array[i] and make region_array[j] None
                // Using this algorithm the region array does not need to be sorted
                if self.region_array[i].as_ref().unwrap().base + self.region_array[i].as_ref().unwrap().size
                    == self.region_array[j].as_ref().unwrap().base
                    && self.region_array[i].as_ref().unwrap().is_available
                    == self.region_array[j].as_ref().unwrap().is_available
                {
                    self.region_array[i].as_mut().unwrap().size += self.region_array[j].as_ref().unwrap().size;
                    self.region_array[j] = None;
                }
                j += 1;
            }
            i += 1;
        }
    }

    /// Returns the index of the smallest region
    fn smallest_region(&self) -> Result<(usize, &Option<PhysicalMemoryRegion>), Error> {
        self.region_array
            .iter()
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

    fn largest_region(&self) -> Result<(usize, &Option<PhysicalMemoryRegion>), Error> {
        self.region_array
            .iter()
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
        self.region_array
            .iter()
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

    fn best_fit_region(&self, size: usize) -> Result<(usize, &Option<PhysicalMemoryRegion>), Error> {
        if self.available_memory() < size {
            if self.available_memory() == 0 {
                Err(Error::OutOfMemory)
            } else {
                Err(Error::InsufficientMemoryAvailable)
            }
        } else {
            self.region_array
                .iter()
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

    pub fn allocate_contiguous(&mut self, count: usize) -> Result<PhysicalMemoryRegion, Error> {
        let alloc_region_index = self.best_fit_region(count * FRAME_SIZE)?.0;
        // Safety: The best_fit_region_index function ensures that the region is available
        let mut alloc_region = unsafe { self.region_array[alloc_region_index].unwrap_unchecked() };
        alloc_region.size -= count * FRAME_SIZE;
        let new_region = PhysicalMemoryRegion::new(
            (alloc_region.base + alloc_region.size - count * FRAME_SIZE) as PhysicalAddress,
            count * FRAME_SIZE,
            false,
        );
        self.add_region(new_region);

        Ok(new_region)
    }

    pub fn deallocate(&mut self, region: PhysicalMemoryRegion) -> Result<(), Error> {
        //find the containing region in the region array
        let containing_region = 
            // Safety: The region array is filtered to ensure the region is in the array.
            // If not the appropriate error is propagated.
            self
                .region_array
                .iter_mut()
                .filter(|region| {
                    if region.is_some() {
                        region.as_ref().unwrap().is_available == false
                    } else {
                        false
                    }
                })
                .find(|r| {
                    if r.is_some() {
                        r.as_ref().unwrap().contains(&region)
                    } else {
                        false
                    }
                })
                .ok_or(Error::AllocatedRegionNotFound)?
                .as_mut()
                .unwrap();
        //deallocate the portion of the region represented by the input region
        let new_region = PhysicalMemoryRegion::new(
            region.base,
            region.size,
            true,
        );
        //determine if the region is at the beginning, middle, or end of the containing region
        //and adjust the containing region accordingly
        if containing_region.base == region.base {
            //if the region is at the beginning, adjust the base and size of the containing region
            containing_region.base += region.size;
            containing_region.size -= region.size;
        } else if containing_region.base + containing_region.size == region.base + region.size {
            //if the region is at the end, adjust the size of the containing region
            containing_region.size -= region.size;
        } else {
            //if the region is in the middle, split the containing region into two regions one before and one after the input region
            *containing_region = PhysicalMemoryRegion {
                base: containing_region.base,
                size: region.base - containing_region.base,
                is_available: false,
            };
            let after_region = PhysicalMemoryRegion::new(
                region.base + region.size,
                containing_region.base + containing_region.size - region.base + region.size,
                false,
            );
            self.add_region(after_region);
        }
        //add the new region to the region array
        self.add_region(new_region);
        //sort the region array
        self.region_array.sort_unstable();

        Ok(())
    }
}

struct PhysicalMemoryManager {
    pfa: PhysicalFrameAllocator,
    unallocated_frames_committed: usize,
}
