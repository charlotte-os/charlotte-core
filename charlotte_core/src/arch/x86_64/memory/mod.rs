use core::arch::x86_64::{CpuidResult, __cpuid_count};
use core::convert::From;
use core::ops::{Index, IndexMut};

use spin::lazy::Lazy;

use crate::arch::MemoryMap;
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

pub enum Error {
    UnsupportedOperation,
    InvalidArgument,
    InvalidAddress,
    InvalidPAddrAlignment,
    InvalidVAddrAlignment,
    UnableToAllocatePageTable,
}

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

    fn get(&self, index: usize) -> PhysicalAddress {
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
    fn set_normal_entry(&mut self, index: usize, paddr: PhysicalAddress, flags: u64) {
        self.entries[index] = paddr.bits() as u64 | flags;
    }

    /// Create an entry in the current table that points to a huge, large, or regular page
    fn set_page_entry(&mut self, index: usize, paddr: PhysicalAddress, flags: u64) {
        self.entries[index] = paddr.bits() as u64 | flags;
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

impl Drop for PageTable {
    fn drop(&mut self) {
        for entry in self.entries.iter() {
            if *entry & 1 == 1 {
                // Drop the page table that this entry points to using head recursion

                // TODO: Call the drop method on the page table that this entry points to
                let paddr = PhysicalAddress::from((*entry & 0x000FFFFF_FFFFF000) as usize);
                let table = unsafe { &mut *(paddr.bits()) };
                // TODO: Free the page frame that the page table is stored in
                // TODO: clear the entry in the current table
            }
        }
    }
}

/// A page map used to map virtual addresses to physical addresses under the x86_64 architecture
pub struct PageMap {
    // The value that will be loaded into the CR3 register when this page map is loaded
    cr3: u64,
}

impl Clone for PageMap {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Drop for PageMap {
    fn drop(&mut self) {
        todo!()
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
