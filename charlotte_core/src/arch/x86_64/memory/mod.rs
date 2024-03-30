use crate::memory::address::{PhysicalAddress, VirtualAddress};
use crate::arch::MemoryMap;

pub struct PageMap {
    pml4_addr: Option<PhysicalAddress>,
}
pub enum Error {
    InvalidAddress,
    InvalidAlignment,
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
        Self {
            pml4_addr: None,
        }
    }

    fn map_page(paddr: PhysicalAddress, vaddr: VirtualAddress) -> Result<(), Self::Error> {
        todo!()
    }

    fn unmap_page(vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
        todo!()
    }

    fn map_large_page(paddr: PhysicalAddress, vaddr: VirtualAddress) -> Result<(), Self::Error> {
        todo!()
    }

    fn unmap_large_page(vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
        todo!()
    }

    fn map_huge_page(paddr: PhysicalAddress, vaddr: VirtualAddress) -> Result<(), Self::Error> {
        todo!()
    }

    fn unmap_huge_page(vaddr: VirtualAddress) -> Result<PhysicalAddress, Self::Error> {
        todo!()
    }
}