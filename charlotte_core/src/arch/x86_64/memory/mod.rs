use core::arch::x86_64::{CpuidResult, __cpuid_count};

use crate::arch::MemoryMap;
use crate::memory::address::{PhysicalAddress, VirtualAddress};
use crate::memory::pmm::*;

pub struct PageMap {
    pml4_addr: Option<PhysicalAddress>,
}
pub enum Error {
    UnsupportedOperation,
    InvalidArgument,
    InvalidAddress,
    InvalidPAddrAlignment,
    InvalidVAddrAlignment,
    UnableToAllocatePageMapTable,
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

impl MemoryMap for PageMap {
    type Error = Error;

    fn new() -> Self {
        Self { pml4_addr: None }
    }

    fn load(&self) -> Result<(), Self::Error> {
        match self.pml4_addr {
            Some(paddr) => {
                unsafe {
                    asm_load_page_map(paddr);
                }
                Ok(())
            }
            None => Err(Error::InvalidArgument),
        }
    }

    fn map_page(
        &mut self,
        paddr: PhysicalAddress,
        vaddr: VirtualAddress,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn unmap_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
        todo!()
    }

    fn map_large_page(
        &mut self,
        paddr: PhysicalAddress,
        vaddr: VirtualAddress,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn unmap_large_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
        todo!()
    }

    fn map_huge_page(
        &mut self,
        paddr: PhysicalAddress,
        vaddr: VirtualAddress,
    ) -> Result<(), Self::Error> {
        /*Validate arguments and handle error cases*/

        // handle the case where huge pages are not supported on the current LP
        if !supports_huge_pages() {
            return Err(Error::UnsupportedOperation);
        }
        // handle the case where the physical address is not aligned to a huge page boundary
        if paddr.0 & 0x1FFFFF != 0 {
            return Err(Error::InvalidPAddrAlignment);
        }
        // handle the case where the virtual address is not aligned to a huge page boundary
        if vaddr.0 & 0x1FFFFF != 0 {
            return Err(Error::InvalidVAddrAlignment);
        }

        /*Map the huge page*/
        // huge pages are mapped in the PML4 table
        if self.pml4_addr.is_none() {
            // allocate a page frame for the PML4 table for this page map
            self.pml4_addr = match PHYSICAL_FRAME_ALLOCATOR.lock().allocate() {
                Ok(paddr) => Some(paddr),
                Err(_) => return Err(Error::UnableToAllocatePageMapTable),
            };
        }
        //set the PML4 entry for the virtual address to the physical address


        Ok(())
    }

    fn unmap_huge_page(&mut self, vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
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

}
