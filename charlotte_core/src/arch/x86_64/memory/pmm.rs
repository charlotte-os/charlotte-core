//! # Physical Memory Manager
//! The physical memory manager manages and ensures safe access to physical memory.
//! It is composed of the physical frame allocator and direct memory map interface.
//! The physical frame allocator provides an interface for allocating and deallocating physical memory frames and
//! contiguous blocks of frames as well as frames that represent MMIO regions.
//! The PFA can be used to allocate and deallocate frames for use by the kernel and user-space applications.
//! It is capable of allocating and deallocating contiguous blocks of frames, which is useful for things like
//! DMA and certain optimization techniques.

use core::mem;

use crate::bootinfo;

use lazy_static::lazy_static;
use limine::memory_map::*;

use spin::mutex::TicketMutex;

// TODO: move this to access ctrl when it is implemented
pub type CapabilityId = u64;

lazy_static! {
    ///This value represents the base virtual address of the direct mapping of physical memory into
    /// kernelspace. It should have the desired physical address added to it and then be cast to a
    /// pointer to access the desired physical address.
    /// Physical addresses should only ever be used while this Mutex is locked.
    /// TODO: Find a way to make this Mutex more fine-grained and function more like a read-write lock on the physical memory.
    pub static ref HHDM_BASE: TicketMutex<usize> = TicketMutex::new(bootinfo::HHDM_REQUEST.get_response().unwrap().offset() as usize);

    ///The physical frame allocator to be used by the kernel and user-space applications.
    pub static ref PFA: TicketMutex<PhysicalFrameAllocator> = TicketMutex::new(PhysicalFrameAllocator::new());
}
/// The number of frames that may need to be allocated but have not been allocated yet.
/// To be used for things like faulted in pages and copy-on-write pages.
/// This value is used to determine how overcommitted the system is.
/// When the system is overcommitted by more than a certain percentage, new allocations will fail.
static UNALLOCATED_FRAMES_COMMITTED: TicketMutex<usize> = TicketMutex::new(0);
/// The percentage of committed frames that must be backed by available frames.
/// When the system is overcommitted by more than this percentage, new allocations will fail.
const REQUIRED_COMMIT_BACKING_PERCENTAGE: usize = 70;

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    InsufficientMemory,
    InsufficientContiguousMemory,
    MemoryOvercommitted,
    PfaRegionArrayFull,
    AllocatedRegionNotFound,
    InvalidArgument,
}

/// Commit frames for future use but do not allocate them yet.
/// This function should be called when frames might be needed in the future but are not needed yet.
#[allow(unused)]
pub fn commit_frames(n_frames: usize) -> Result<(), Error> {
    if PFA.lock().get_commitment_coverage() >= REQUIRED_COMMIT_BACKING_PERCENTAGE {
        return Err(Error::MemoryOvercommitted);
    }

    let mut unallocated_frames_committed = UNALLOCATED_FRAMES_COMMITTED.lock();
    *unallocated_frames_committed += n_frames;
    Ok(())
}
/// Uncommit frames previously committed but not allocated
/// This function should be called when frames are actually allocated to fulfil the commitment or when the commitment is
/// released
#[allow(unused)]
pub fn uncommit_frames(n_frames: usize) -> Result<(), Error> {
    let mut unallocated_frames_committed = UNALLOCATED_FRAMES_COMMITTED.lock();
    if n_frames > *unallocated_frames_committed {
        Err(Error::InvalidArgument)
    } else {
        *unallocated_frames_committed -= n_frames;
        Ok(())
    }
}

///This enum represents the different types of physical memory regions that the PFA manages.
/// It is identical to the defines used by Limine with the exception of PfaReserved, which is used to represent
/// regions of physical memory that are reserved for use by the PFA itself and PfaNull, which is used to represent
/// region descriptors that are not in use.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct PhysicalMemoryRegion {
    capability: Option<CapabilityId>,
    base: usize,
    n_frames: usize,
    region_type: PhysicalMemoryType,
}

impl PhysicalMemoryRegion {
    fn is_less(a: &PhysicalMemoryRegion, b: &PhysicalMemoryRegion) -> bool {
        if a.region_type == PhysicalMemoryType::PfaNull
            && b.region_type != PhysicalMemoryType::PfaNull
        {
            //Null descriptors are always less than any other descriptor
            true
        } else if a.region_type != PhysicalMemoryType::PfaNull
            && b.region_type == PhysicalMemoryType::PfaNull
        {
            //Null descriptors are always less than any other descriptor
            false
        } else if a.region_type == PhysicalMemoryType::PfaNull
            && b.region_type == PhysicalMemoryType::PfaNull
        {
            // When both descriptors are null, they are equal
            false
        } else {
            // If neither descriptor is null, compare their base addresses
            a.base < b.base
        }
    }
}

impl Ord for PhysicalMemoryRegion {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if PhysicalMemoryRegion::is_less(self, other) {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

/// The physical frame allocator is responsible for managing and allocating physical memory frames.
#[derive(Debug)]
pub struct PhysicalFrameAllocator {
    region_array_base: usize, // physical base address of the array of physical memory regions
    region_array_len: usize,  // number of elements in the array of physical memory regions
}

/// The average number of frames that are allocated at a time.
/// This value is used to determine the initial size of the physical memory region array.
/// The initial size of the array is equal to the total amount of physical frames divided by this value.
const ALLOCATION_FACTOR: usize = 50;

/// The size of each frame in bytes.
const FRAME_SIZE: usize = 4096;

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
            > largest_usable_region.length as usize / mem::size_of::<PhysicalMemoryRegion>()
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
            if entry.entry_type != EntryType::BAD_MEMORY {
                total_memory += entry.length as usize;
            }
        }
        total_memory / FRAME_SIZE
    }

    /// Returns the largest usable memory region in the memory map.
    fn get_largest_usable_region(memory_map: &'static [&Entry]) -> Option<&'static Entry> {
        let mut largest_usable_region: Option<&Entry> = None;
        for entry in memory_map {
            if entry.entry_type == EntryType::USABLE {
                match largest_usable_region {
                    Some(lur) => {
                        if entry.length > lur.length {
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
            (self.region_array_base + *HHDM_BASE.lock()) as *const PhysicalMemoryRegion,
            self.region_array_len,
        )
    }
    /// Returns the physical memory region array as a mutable slice.
    unsafe fn get_mut_region_array(&mut self) -> &mut [PhysicalMemoryRegion] {
        core::slice::from_raw_parts_mut(
            (self.region_array_base + *HHDM_BASE.lock()) as *mut PhysicalMemoryRegion,
            self.region_array_len,
        )
    }

    /// Initializes the physical memory region array using the memory map.
    fn init(&mut self, memory_map: &[&Entry]) {
        if self.region_array_len < memory_map.len() {
            panic!("The initial size of the physical memory region array is less than the number of memory map entries.\n
            modifying the ALLOCATION_FACTOR constant may fix this issue.");
        }

        let region_array = unsafe { self.get_mut_region_array() };
        //initialize the region array using the memory map
        for (i, entry) in memory_map.iter().enumerate() {
            if entry.entry_type != EntryType::BAD_MEMORY {
                region_array[i] = PhysicalMemoryRegion {
                    capability: None,
                    base: entry.base as usize,
                    n_frames: entry.length as usize / FRAME_SIZE,
                    region_type: match entry.entry_type {
                        EntryType::USABLE => PhysicalMemoryType::Usable,
                        EntryType::RESERVED => PhysicalMemoryType::Reserved,
                        EntryType::ACPI_RECLAIMABLE => PhysicalMemoryType::AcpiReclaimable,
                        EntryType::ACPI_NVS => PhysicalMemoryType::AcpiNvs,
                        EntryType::BOOTLOADER_RECLAIMABLE => {
                            PhysicalMemoryType::BootloaderReclaimable
                        }
                        EntryType::KERNEL_AND_MODULES => PhysicalMemoryType::KernelAndModules,
                        EntryType::FRAMEBUFFER => PhysicalMemoryType::FrameBuffer,
                        _ => PhysicalMemoryType::BadMemory,
                    },
                };
            }
        }
        //initialize the rest of the region array with null descriptors
        for i in memory_map.len()..region_array.len() {
            region_array[i] = PhysicalMemoryRegion {
                capability: None,
                base: 0,
                n_frames: 0,
                region_type: PhysicalMemoryType::PfaNull,
            };
        }
        // add the region that represents the physical memory region array itself
        let pfa_region = PhysicalMemoryRegion {
            capability: None,
            base: region_array.as_ptr() as usize - *HHDM_BASE.lock(), // this is here because borrowing rules prevent us from using self.region_array_base
            n_frames: region_array.len(),
            region_type: PhysicalMemoryType::PfaReserved,
        };
        for region in region_array.iter_mut() {
            if region.region_type == PhysicalMemoryType::PfaNull {
                *region = pfa_region;
                break;
            }
        }
        // Merge adjacent regions of the same type and sort the region array by base address
        Self::merge_and_sort_region_array(region_array);
    }

    /// Merge adjacent regions of the same type and sort the region array by base address.
    fn merge_and_sort_region_array(region_array: &mut [PhysicalMemoryRegion]) {
        let mut next_nonnull_index: usize;
        //Merge adjacent regions of the same type
        'array_loop: for i in 0..region_array.len() {
            //find the next non-null region
            next_nonnull_index = i + 1;

            while next_nonnull_index < region_array.len()
                && region_array[next_nonnull_index].region_type == PhysicalMemoryType::PfaNull
            {
                next_nonnull_index += 1;
            }

            if next_nonnull_index == region_array.len() {
                break 'array_loop;
            }

            //if the current region and the next region are of the same type, not null, and adjacent, merge them
            if region_array[i].region_type == region_array[next_nonnull_index].region_type
                && region_array[i].region_type != PhysicalMemoryType::PfaNull
                && region_array[i].base + region_array[i].n_frames * FRAME_SIZE
                    == region_array[next_nonnull_index].base
            {
                region_array[i].n_frames += region_array[next_nonnull_index].n_frames;
                region_array[next_nonnull_index].region_type = PhysicalMemoryType::PfaNull;
            }
        }

        //Sort the region array by base address and move all null regions to the end of the array
        // quicksort(region_array, &PhysicalMemoryRegion::is_less);
        region_array.sort_unstable()
    }

    fn append_region(
        region_array: &mut [PhysicalMemoryRegion],
        region: PhysicalMemoryRegion,
    ) -> Result<(), Error> {
        for i in 0..region_array.len() {
            if region_array[i].region_type == PhysicalMemoryType::PfaNull {
                region_array[i] = region;
                Self::merge_and_sort_region_array(region_array);
                return Ok(());
            }
        }
        Err(Error::PfaRegionArrayFull)
    }

    /// Allocate a contiguous block of physical memory frames.
    pub fn allocate_frames(
        &mut self,
        n_frames: usize,
        capability: Option<CapabilityId>,
    ) -> Result<PhysicalMemoryRegion, Error> {
        //check if the system is overcommitted
        if self.get_commitment_coverage() > REQUIRED_COMMIT_BACKING_PERCENTAGE {
            return Err(Error::MemoryOvercommitted);
        }

        let region_array = unsafe { self.get_mut_region_array() };
        let mut smallest_usable_region = Option::<&mut PhysicalMemoryRegion>::None;
        //find the smallest usable region that can hold the requested number of frames
        //this is done to minimize fragmentation
        for region in region_array.iter_mut() {
            if region.region_type == PhysicalMemoryType::Usable && region.n_frames >= n_frames {
                match &smallest_usable_region {
                    Some(sur) => {
                        if region.n_frames < sur.n_frames {
                            smallest_usable_region = Some(region);
                        }
                    }
                    None => {
                        smallest_usable_region = Some(region);
                    }
                }
            }
        }
        match smallest_usable_region {
            //if a suitable region was found, allocate the frames and update the region array
            Some(sur) => {
                let allocated_region = PhysicalMemoryRegion {
                    capability,
                    base: sur.base,
                    n_frames,
                    region_type: PhysicalMemoryType::Allocated,
                };
                sur.base += n_frames * FRAME_SIZE;
                sur.n_frames -= n_frames;
                Self::append_region(region_array, allocated_region.clone())?;
                Ok(allocated_region)
            }
            //if no suitable region was found, return an error
            None => Err(Error::InsufficientContiguousMemory),
        }
    }
    /// Deallocate a previously allocated contiguous block of physical memory frames.
    pub fn deallocate_frames(&mut self, region: PhysicalMemoryRegion) -> Result<(), Error> {
        let region_array = unsafe { self.get_mut_region_array() };
        for r in region_array.iter_mut() {
            if r.base == region.base
                && r.n_frames == region.n_frames
                && r.region_type == PhysicalMemoryType::Allocated
            {
                r.region_type = PhysicalMemoryType::Usable;
                r.capability = None;
                Self::merge_and_sort_region_array(region_array);
                return Ok(());
            }
        }
        Err(Error::AllocatedRegionNotFound)
    }
    /// Obtain the number of available physical memory frames.
    pub fn get_n_available_frames(&self) -> usize {
        let region_array = unsafe { self.get_region_array() };
        let mut n_available_frames = 0;
        for region in region_array.iter() {
            if region.region_type == PhysicalMemoryType::Usable {
                n_available_frames += region.n_frames;
            }
        }
        n_available_frames
    }
    /// Determine the commitment coverage of the system.
    /// The commitment coverage is the percentage of committed frames that are backed by available frames.
    pub fn get_commitment_coverage(&self) -> usize {
        let available_frames = self.get_n_available_frames();
        let commited_frames = UNALLOCATED_FRAMES_COMMITTED.lock();
        (*commited_frames / available_frames) * 100
    }
}
