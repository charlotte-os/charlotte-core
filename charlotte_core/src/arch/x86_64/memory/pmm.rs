//! # Physical Memory Manager
//! The physical memory manager manages and ensures safe access to physical memory.
//! It is composed of the physical frame allocator and direct memory map interface.
//! The physical frame allocator provides an interface for allocating and deallocating physical memory frames and
//! contiguous blocks of frames as well as frames that represent MMIO regions.
//! The PFA can be used to allocate and deallocate frames for use by the kernel and user-space applications.
//! It is capable of allocating and deallocating contiguous blocks of frames, which is useful for things like
//! DMA and certain optimization techniques.
//! The direct memory map interface provides a means for safely accessing physical memory via a direct mapping
//! region. It is located in its own module and is documented there.

use core::borrow::{Borrow, BorrowMut};
use core::mem;
use core::slice::sort::quicksort;

use crate::access_control::CapabilityId;
use crate::bootinfo;

use lazy_static::lazy_static;
use limine::memory_map::*;

use spin::mutex::TicketMutex;

lazy_static! {
    ///This value represents the base virtual address of the direct mapping of physical memory into
    /// kernelspace. It should have the desired physical address added to it and then be cast to a 
    /// pointer to access the desired physical address.
    /// Physical addresses should only ever be used while this Mutex is locked.
    /// TODO: Find a way to make this Mutex more fine-grained and function more like a read-write lock on the physical memory.
    pub static ref HHDM_BASE: TicketMutex<u64> = TicketMutex::new(bootinfo::HHDM_REQUEST.get_response().unwrap().offset());

    ///The physical frame allocator to be used by the kernel and user-space applications.
    pub static ref PFA: TicketMutex<PhysicalFrameAllocator> = TicketMutex::new(PhysicalFrameAllocator::new());
}

pub enum Error {
    InsufficientMemory,
    InsufficientContiguousMemory,
    PfaRegionArrayFull,
    AllocatedRegionNotFound,
}

///This enum represents the different types of physical memory regions that the PFA manages.
/// It is identical to the defines used by Limine with the exception of PfaReserved, which is used to represent
/// regions of physical memory that are reserved for use by the PFA itself and PfaNull, which is used to represent
/// region descriptors that are not in use.
pub enum PhysicalMemoryType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    KernelAndModules,
    FrameBuffer,
    Allocated,
    PfaReserved,
    PfaNull,
}

/// A physical memory region descriptor. This struct is used to represent a region of physical memory in the
/// physical memory region array. It contains the base address of the region, the number of frames in the region,
/// the type of the region, and optionally a capability capability that is used to access the region.
#[derive(Clone)]
pub struct PhysicalMemoryRegion {
    capability: Option<CapabilityId>,
    base: usize,
    n_frames: usize,
    region_type: PhysicalMemoryType,
}

impl PhysicalMemoryRegion {
    fn is_less(a: &PhysicalMemoryRegion, b: &PhysicalMemoryRegion) -> bool {
        if a.region_type == PhysicalMemoryType::PfaNull && b.region_type != PhysicalMemoryType::PfaNull {
        //Null descriptors are always less than any other descriptor
            true
        } else if a.region_type != PhysicalMemoryType::PfaNull && b.region_type == PhysicalMemoryType::PfaNull {
        //Null descriptors are always less than any other descriptor
            false
        } else if a.region_type == PhysicalMemoryType::PfaNull && b.region_type == PhysicalMemoryType::PfaNull {
        // When both descriptors are null, they are equal
            false
        } else {
        // If neither descriptor is null, compare their base addresses
            a.base < b.base
        }
    }
}

/// The physical frame allocator is responsible for managing and allocating physical memory frames.
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

    /// Returns the physical memory region array as a slice.
    unsafe fn get_region_array(&self) -> &[PhysicalMemoryRegion] {
        core::slice::from_raw_parts(
            (self.region_array_base + *(HHDM_BASE.borrow())) as *const PhysicalMemoryRegion,
            self.region_array_len,
        )
    }
    /// Returns the physical memory region array as a mutable slice.
    unsafe fn get_mut_region_array(&self) -> &mut [PhysicalMemoryRegion] {
        core::slice::from_raw_parts_mut(
            (self.region_array_base + *(HHDM_BASE.borrow_mut())) as *mut PhysicalMemoryRegion,
            self.region_array_len,
        )
    }

    /// Initializes the physical memory region array using the memory map.
    fn init(&mut self, memory_map: &[&Entry]) {
        if self.region_array_len < memory_map.len() {
            panic!("The initial size of the physical memory region array is less than the number of memory map entries.\n
            modifying the ALLOCATION_FACTOR constant may fix this issue.");
        }

        let mut region_array = unsafe { self.get_mut_region_array() };
        for (i, entry) in memory_map.enumerate() {
            if entry.region_type != EntryType::BadMemory {
                region_array[i] = PhysicalMemoryRegion {
                    capability: None,
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
                capability: None,
                base: 0,
                n_frames: 0,
                region_type: PhysicalMemoryType::PfaNull,
            };
        }
        // add the region that represents the physical memory region array itself
        let pmm_region = PhysicalMemoryRegion {
            capability: None,
            base: self.region_array_base,
            n_frames: self.region_array_len,
            region_type: PhysicalMemoryType::PfaReserved,
        };
        for region in region_array.iter() {
            if region.region_type == PhysicalMemoryType::PfaNull {
                *region = pmm_region;
                break;
            }
        }
        // Correct the region array
        self.merge_and_sort_region_array();
    }

    /// Merge adjacent regions of the same type and sort the region array by base address.
    fn merge_and_sort_region_array(&mut self) {
        let mut region_array = unsafe { self.get_mut_region_array() };
        
        //Merge adjacent regions of the same type
        'array_loop: for i in 0..self.region_array_len {
            //find the next non-null region
            let mut next_nonnull_index = i + 1;
            while region_array[next_nonnull_index].region_type == PhysicalMemoryType::PfaNull {
                next_nonnull_index += 1;
            }
            //if there are no more non-null regions, break
            if next_nonnull_index == self.region_array_len {
                break 'array_loop;
            }
            //if the current region and the next region are of the same type, not null, and adjacent, merge them                 
            if region_array[i].region_type == region_array[next_nonnull_index].region_type && region_array[i].region_type != PhysicalMemoryType::PfaNull {
                if region_array[i].base + region_array[i].n_frames * 4096 == region_array[next_nonnull_index].base {
                    region_array[i].n_frames += region_array[next_nonnull_index].n_frames;
                    region_array[next_nonnull_index].region_type = PhysicalMemoryType::PfaNull;
                }
            }
        }

        //Sort the region array by base address and move all null regions to the end of the array
        quicksort(region_array, &PhysicalMemoryRegion::is_less);
    }

    fn append_region(&mut self, region: PhysicalMemoryRegion) -> Result<(), Error> {
        let mut region_array = unsafe { self.get_mut_region_array() };
        for i in 0..self.region_array_len {
            if region_array[i].region_type == PhysicalMemoryType::PfaNull {
                region_array[i] = region;
                self.merge_and_sort_region_array();
                Ok(())
            }
        }
        Err(Error::PfaRegionArrayFull)
    }

    /// Allocate a contiguous block of physical memory frames.
    pub fn allocate_frames(&mut self, n_frames: usize, capability: Option<CapabilityId>) -> Result<PhysicalMemoryRegion, Error> {
        let mut region_array = unsafe { self.get_mut_region_array() };
        for region in region_array.iter_mut() {
            if region.region_type == PhysicalMemoryType::Usable && region.n_frames >= n_frames {
                //Create the descriptor for the newly allocated region
                let mut allocated_region = PhysicalMemoryRegion {
                    capability: capability,
                    base: region.base,
                    n_frames,
                    region_type: PhysicalMemoryType::Allocated,
                };
                //Add the allocated region descriptor to the region array
                //This also merges adjacent regions of the same type and sorts the region array
                self.append_region(allocated_region.clone())?;
                //Update the region descriptor for the region that was allocated from
                region.base += n_frames * 4096;
                region.n_frames -= n_frames;

                Ok(allocated_region)
            }
        }
        Err(Error::InsufficientContiguousMemory)
    }
    /// Deallocate a previously allocated contiguous block of physical memory frames.
    pub fn deallocate_frames(&mut self, region: PhysicalMemoryRegion) -> Result<(), Error> {
        let mut region_array = unsafe { self.get_mut_region_array() };
        for r in region_array.iter_mut() {
            if r.base == region.base && r.n_frames == region.n_frames {
                r.region_type = PhysicalMemoryType::Usable;
                r.capability = None;
                self.merge_and_sort_region_array();
                Ok(())
            }
        }
        Err(Error::AllocatedRegionNotFound)
    }
}
