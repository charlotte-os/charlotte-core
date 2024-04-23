use core::arch::x86_64::__cpuid_count;
use core::ops::{Index, IndexMut};

use spin::lazy::Lazy;

use crate::memory::address::{PhysicalAddress, VirtualAddress};
use crate::memory::pmm::*;

// Determines whether 5 level or 4 level paging is in use
// As of now 5 level paging is not supported
// const USE_5_LEVEL_PAGING: bool = false;

/// The number of significant binary digits in a physical address
pub static PADDR_SIGBITS: Lazy<u8> = Lazy::new(|| {
    let cpuid_result = unsafe { __cpuid_count(0x80000008, 0) };
    (cpuid_result.eax & 0xFF) as u8
});

/// This is the number of significant binary digits in a virtual (linear) address
/// This will not be used for now however it is here to support an implementation of
/// 5-level paging in the future
pub static VADDR_SIGBITS: Lazy<u8> = Lazy::new(|| {
    let cpuid_result = unsafe { __cpuid_count(0x80000001, 0) };
    if cpuid_result.ecx & (1 << 16) != 0 {
        57u8
    } else {
        48u8
    }
});

static ADDR_MASK: Lazy<u64> = Lazy::new(|| {
    // Create a mask that will clear all bits in a PTE that are not address bits
    let mut mask = (1 << *PADDR_SIGBITS) - 1;
    mask &= !0xFFF; // Clear the lower 12 bits
    mask
});

pub enum Error {
    UnsupportedOperation,
    InvalidArgument,
    InvalidAddress,
    InvalidPAddrAlignment,
    InvalidVAddrAlignment,
    UnableToAllocatePageTable,
}

enum PteFlags {
    Present = 1,
    Write = 1 << 1,
    User = 1 << 2,
    WriteThrough = 1 << 3,
    CacheDisable = 1 << 4,
    Accessed = 1 << 5,
    Dirty = 1 << 6,    // Only for entries that point to pages
    PageSize = 1 << 7, // Only for entires in the PDPT, and PD for 1G and 2M pages
    Global = 1 << 8,
    NoExecute = 1 << 63,
}

static FLAG_MASK: u64 = PteFlags::Present as u64
    | PteFlags::Write as u64
    | PteFlags::User as u64
    | PteFlags::WriteThrough as u64
    | PteFlags::CacheDisable as u64
    | PteFlags::Accessed as u64
    | PteFlags::Dirty as u64
    | PteFlags::PageSize as u64
    | PteFlags::Global as u64
    | PteFlags::NoExecute as u64;

#[repr(transparent)]
struct PageTable {
    entries: [u64; 512],
}

impl PageTable {
    fn new() -> Self {
        Self {
            entries: [0u64; 512],
        }
    }

    fn map(&mut self, index: usize, paddr: PhysicalAddress, flags: u64) -> Result<(), Error> {
        if paddr.bits() & 0xFFF != 0 {
            Err(Error::InvalidPAddrAlignment)
        } else {
            self.entries[index] = paddr.bits() as u64 | flags;
            Ok(())
        }
    }

    fn unmap(&mut self, index: usize) -> PhysicalAddress {
        let frame = PhysicalAddress::from((self.entries[index] & 0x000FFFFF_FFFFF000) as usize);
        self.entries[index] = 0;
        frame
    }

    fn get_paddr(&self, index: usize) -> PhysicalAddress {
        PhysicalAddress::from((self.entries[index] & 0x000FFFFF_FFFFF000) as usize)
    }

    fn is_present(&self, index: usize) -> bool {
        self.entries[index] & 1 == 1
    }

    fn set_present_bit(&mut self, index: usize, present: bool) {
        let entry = &mut self.entries[index];
        if present {
            *entry |= 1;
        } else {
            *entry &= !1;
        }
    }

    fn is_size_bit_set(&self, index: usize) -> bool {
        self.entries[index] & (1 << 7) == 1
    }

    fn set_size_bit(&mut self, index: usize, size: bool) {
        let entry = &mut self.entries[index];
        if size {
            *entry |= 1 << 7;
        } else {
            *entry &= !(1 << 7);
        }
    }

    /// Clears the entry at the given index
    /// # Safety
    /// This function is unsafe because it can clear an entry without dropping the lower level page table
    /// that it points to and all of its descendants which would be a memory leak
    /// This function should only be called when the page table being pointed to has been deallocated
    /// TODO: Implement a safe wrapper as a method of the PageMap struct
    unsafe fn clear_entry(&mut self, index: usize) {
        self.entries[index] = 0u64;
    }

    /// Create an entry in the current table that points to the next table down the hierarchy
    /// CURRENT IMPLEMENTATION IS WRONG
    fn set_entry(&mut self, index: usize, paddr: PhysicalAddress, flags: u64) {
        let corrected_flags = flags & FLAG_MASK;
        let corrected_paddr = paddr.bits() as u64 & *ADDR_MASK;
        self.entries[index] = corrected_paddr | corrected_flags;
    }
}

impl Index<usize> for PageTable {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

/// A page map used to map virtual addresses to physical addresses under the x86_64 architecture
pub struct PageMap {
    // The value that will be loaded into the CR3 register when this page map is loaded
    cr3: u64,
}

impl Clone for PageMap {
    /// Perform an explicit deep copy of the page map
    ///
    /// Warning: This is a computationally expensive operation
    fn clone(&self) -> Self {
        // Create a new page map cr3 value
        // Set the flags equal to the flags of the current page map
        let mut cr3 = self.cr3 & 0xFFF;
        // Check to see if the current page map has a PML4 table
        if self.cr3 & !0xFFF != 0 {
            // Allocate a frame for the new PML4 table
            let new_pml4_paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate().unwrap();
            // Set the upeer 48 bits of the cr3 value to point to the new PML4 table
            cr3 |= new_pml4_paddr.bits() as u64 & !0xFFF;
        }
        // Replicate every PML4 entry down the hierarchy
        let self_pml4_paddr = PhysicalAddress::from((self.cr3 & !0xFFF) as usize);
        let self_pml4_table =
            unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + self_pml4_paddr.bits())) };
        let pml4_paddr = PhysicalAddress::from((self.cr3 & !0xFFF) as usize);
        let pml4_table = unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + pml4_paddr.bits())) };

        for i4 in 0..512 {
            if self_pml4_table.is_present(i4) {
                let self_pdpt_paddr = self_pml4_table.get_paddr(i4);
                let self_pdpt_table =
                    unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + self_pdpt_paddr.bits())) };
                let pdpt_paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate().unwrap();
                pml4_table.map(i4, pdpt_paddr, self_pml4_table[i4] & FLAG_MASK);
                let pdpt_table =
                    unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + pdpt_paddr.bits())) };

                for ipdpt in 0..512 {
                    if self_pdpt_table.is_present(ipdpt) {
                        let self_pd_paddr = self_pdpt_table.get_paddr(ipdpt);
                        let self_pd_table = unsafe {
                            &mut *(<*mut PageTable>::from(*DIRECT_MAP + self_pd_paddr.bits()))
                        };
                        let pd_paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate().unwrap();
                        pdpt_table.map(ipdpt, pd_paddr, self_pdpt_table[ipdpt] & FLAG_MASK);
                        let pd_table = unsafe {
                            &mut *(<*mut PageTable>::from(*DIRECT_MAP + pd_paddr.bits()))
                        };

                        if pdpt_table.is_size_bit_set(ipdpt) == false {
                            for ipd in 0..512 {
                                if self_pd_table.is_present(ipd) {
                                    let self_pt_paddr = self_pd_table.get_paddr(ipd);
                                    let self_pt_table = unsafe {
                                        &mut *(<*mut PageTable>::from(
                                            *DIRECT_MAP + self_pt_paddr.bits(),
                                        ))
                                    };
                                    let pt_paddr =
                                        PHYSICAL_FRAME_ALLOCATOR.lock().allocate().unwrap();
                                    pd_table.map(ipd, pt_paddr, self_pd_table[ipd] & FLAG_MASK);
                                    let pt_table = unsafe {
                                        &mut *(<*mut PageTable>::from(
                                            *DIRECT_MAP + pt_paddr.bits(),
                                        ))
                                    };

                                    if pd_table.is_size_bit_set(ipd) == false {
                                        for ipt in 0..512 {
                                            if self_pt_table.is_present(ipt) {
                                                // if the page is present then map the page to the new page table by copying the entry
                                                pt_table[ipt] = self_pt_table[ipt];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        PageMap { cr3 }
    }
}

impl Drop for PageMap {
    /// Drop the page map and deallocate all of the page tables that constitute it.
    ///
    /// This destructor does not deallocate the frames that the page tables point to
    /// because the page tables are not the owners of the frames, the capabilities with
    /// which the frames were allocated are the owners of the frames and they will deallocate
    /// the frames when they are dropped.
    /// This is because the same frames can be accesses from multiple address spaces and
    /// just because a frame is unmapped from an address space does not mean that the process that did so
    /// may not want to map the frame again in the future using the capability the owns it
    fn drop(&mut self) {
        // Check to see if the cr3 value point to a PML4 table
        // If it does not then return
        if !0xFFF & self.cr3 == 0 {
            return;
        } else {
            // iterate through the PML4 table and drop all of the tables that it points to recursively
            let pml4_paddr = PhysicalAddress::from((self.cr3 & !0xFFF) as usize);
            let pml4_table =
                unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + pml4_paddr.bits())) };

            for i in 0..512 {
                if pml4_table.is_present(i) {
                    let pdpt_paddr = pml4_table.get_paddr(i);
                    let pdpt_table =
                        unsafe { &mut *(<*mut PageTable>::from(*DIRECT_MAP + pdpt_paddr.bits())) };

                    for j in 0..512 {
                        if pdpt_table.is_present(j) {
                            let pd_paddr = pdpt_table.get_paddr(j);
                            let pd_table = unsafe {
                                &mut *(<*mut PageTable>::from(*DIRECT_MAP + pd_paddr.bits()))
                            };

                            for k in 0..512 {
                                if pd_table.is_present(k) {
                                    let pt_paddr = pd_table.get_paddr(k);
                                    let pt_table = unsafe {
                                        &mut *(<*mut PageTable>::from(
                                            *DIRECT_MAP + pt_paddr.bits(),
                                        ))
                                    };

                                    PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(pt_paddr);
                                }
                            }

                            PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(pd_paddr);
                        }
                    }

                    PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(pdpt_paddr);
                }
            }
        }
    }
}

/// Determines whether the current LP supports huge pages.
/// Returns `true` if huge pages are supported, `false` otherwise.
fn supports_huge_pages() -> bool {
    let cpuid_result = unsafe { __cpuid_count(0x80000001, 0) };
    let edx = cpuid_result.edx;
    edx & (1 << 26) != 0
}

extern "C" {
    fn asm_load_page_map(paddr: PhysicalAddress);
    fn asm_invalidate_tlb_entry(vaddr: VirtualAddress);
    pub fn asm_get_cr4() -> u64;
}
