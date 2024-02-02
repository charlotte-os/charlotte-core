//! # Physical Frame Allocator
//! The physical frame allocator provides an interface for allocating and deallocating physical memory frames and
//! contiguous blocks of frames as well as frames that represent MMIO regions.
//! The PFA can be used to allocate and deallocate frames for use by the kernel and user-space applications.
//! It is capable of allocating and deallocating contiguous blocks of frames, which is useful for things like
//! DMA and certain optimization techniques.

use core::mem;

use crate::access_control::Capability;
use crate::bootinfo;

use limine::memory_map::*;

///This enum represents the different types of physical memory regions that the PFA manages.
/// It is identical to the defines used by Limine with the exception of PfaReserved, which is used to represent
/// regions of physical memory that are reserved for use by the PFA itself and PfaNull, which is used to represent
/// region descriptors that are not in use.
enum PhysicalMemoryType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    KernelAndModules,
    FrameBuffer,
    PfaReserved,
    PfaNull,
}

struct PhysicalMemoryRegion<'a> {
    key: Option<&'a dyn Capability>,
    base: usize,
    n_frames: usize,
    region_type: PhysicalMemoryType,
}

pub struct PhysicalFrameAllocator {
    region_array_base: usize, // physical base address of the array of physical memory regions
    region_array_len: usize,  // number of elements in the array of physical memory regions
}

/// The average number of frames that are allocated at a time.
/// This value is used to determine the initial size of the physical memory region array.
/// The initial size of the array is equal to the total amount of physical frames divided by this value.
const ALLOCATION_FACTOR: usize = 100;

impl PhysicalFrameAllocator {
    /// Constructs a new PhysicalFrameAllocator and initializes it using the memory map.
    /// # Panics
    /// This function will panic if no usable memory regions are found in the memory map.
    /// This function will panic if no linear memory region large enough to hold the initial physical memory region array is found.
    fn new() -> PhysicalFrameAllocator {
        let memory_map = bootinfo::MEMORY_MAP_REQUEST
            .get_response()
            .unwrap()
            .entries();
        let largest_usable_region = match Self::get_largest_usable_region(memory_map) {
            Some(lur) => lur,
            None => panic!("No usable memory regions found"),
        };
        let init_region_array_len = Self::detect_total_frames(memory_map) / ALLOCATION_FACTOR;
        if init_region_array_len
            > largest_usable_region.length / mem::size_of::<PhysicalMemoryRegion>()
        {
            panic!("No linear memory region large enough to hold the initial phyiscal memory region array");
        }

        let mut pfa = PhysicalFrameAllocator {
            region_array_base: largest_usable_region.base as usize,
            region_array_len: init_region_array_len,
        };

        pfa.init(memory_map);

        pfa
    }
    /// Detects the total amount of functional memory in the system whether it is usable or not.
    fn detect_total_frames(memory_map: &[&Entry]) -> usize {
        let mut total_memory = 0;
        for entry in memory_map {
            if entry.region_type != EntryType::BadMemory {
                total_memory += entry.region_length as usize;
            }
        }
        total_memory / 4096
    }
    /// Returns the largest usable memory region in the memory map.
    fn get_largest_usable_region(memory_map: &[&Entry]) -> Option<&'static Entry> {
        let mut largest_usable_region: Option<&Entry> = None;
        for entry in memory_map {
            if entry.entry_type == EntryType::Usable {
                match largest_usable_region {
                    Some(lur) => {
                        if entry.region_length > lur.region_length {
                            largest_usable_region = Some(entry);
                        }
                    }
                    None => {
                        largest_usable_region = Some(entry);
                    }
                }
            }
        }
        largest_usable_region
    }
    /// Initializes the physical memory region array using the memory map.
    fn init(&mut self, memory_map: &[&Entry]) {
        let mut region_array = unsafe {
            core::slice::from_raw_parts_mut(
                (self.region_array_base + *super::HHDM_BASE) as *mut PhysicalMemoryRegion,
                self.region_array_len,
            )
        };
        for (i, entry) in memory_map.enumerate() {
            if entry.region_type != EntryType::BadMemory {
                region_array[i] = PhysicalMemoryRegion {
                    key: None,
                    base: entry.base as usize,
                    n_frames: entry.length / 4096 as usize,
                    region_type: match entry.region_type {
                        EntryType::Usable => PhysicalMemoryType::Usable,
                        EntryType::Reserved => PhysicalMemoryType::Reserved,
                        EntryType::AcpiReclaimable => PhysicalMemoryType::AcpiReclaimable,
                        EntryType::AcpiNvs => PhysicalMemoryType::AcpiNvs,
                        EntryType::BootloaderReclaimable => {
                            PhysicalMemoryType::BootloaderReclaimable
                        }
                        EntryType::KernelAndModules => PhysicalMemoryType::KernelAndModules,
                        EntryType::FrameBuffer => PhysicalMemoryType::FrameBuffer,
                        _ => PhysicalMemoryType::BadMemory,
                    },
                };
            }
        }
        for i in memory_map.len()..self.region_array_len {
            region_array[i] = PhysicalMemoryRegion {
                key: None,
                base: 0,
                n_frames: 0,
                region_type: PhysicalMemoryType::PfaNull,
            };
        }
        //TODO: Sort the region array by base address
        //TODO: Merge adjacent regions of the same type
        //TODO: Mark regions that are reserved for use by the PFA
    }
}
