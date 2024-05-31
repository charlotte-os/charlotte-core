mod page_table;

use page_table::PageTable;

use super::Error;

use core::arch::asm;
use core::num::NonZeroUsize;
use core::ptr::addr_of_mut;

use crate::arch::x86_64::cpu::ARE_HUGE_PAGES_SUPPORTED;
use crate::arch::{Api, ArchApi};
use crate::memory::address::VirtualAddress;
use crate::memory::{address::PhysicalAddress, pmm::PHYSICAL_FRAME_ALLOCATOR};

struct Walker<'a> {
    page_map: &'a PageMap,
    pml4: Option<&'a mut PageTable>,
    pdpt: Option<&'a mut PageTable>,
    pd: Option<&'a mut PageTable>,
    pt: Option<&'a mut PageTable>,
}

impl<'a> Walker<'a> {
    fn new(page_map: &'a PageMap) -> Self {
        Self {
            page_map: page_map,
            pml4: None,
            pdpt: None,
            pd: None,
            pt: None,
        }
    }
    fn walk_cr3(&mut self) -> Result<(), Error> {
        unsafe {
            self.pml4 = Some(&mut *(<*mut PageTable>::from(self.page_map.get_pml4_paddr())));
        }
        Ok(())
    }
    fn walk_pml4(&mut self, vaddr: VirtualAddress, flags: u64) -> Result<(), Error> {
        match &mut self.pml4 {
            Some(pml4) => {
                unsafe {
                    let pml4_ptr = addr_of_mut!(*pml4);
                    self.pdpt = Some(
                        &mut *((*pml4_ptr).get_or_map_table(
                            vaddr,
                            page_table::PageTableLevel::PML4,
                            flags,
                        )?),
                    );
                }
                Ok(())
            }
            None => {
                self.walk_cr3()?;
                self.walk_pml4(vaddr, flags)
            }
        }
    }

    fn walk_pdpt(&mut self, vaddr: VirtualAddress, flags: u64) -> Result<(), Error> {
        match &mut self.pdpt {
            Some(pdpt) => {
                unsafe {
                    let pdpt_ptr = addr_of_mut!(*pdpt);
                    self.pd = Some(
                        &mut *((*pdpt_ptr).get_or_map_table(
                            vaddr,
                            page_table::PageTableLevel::PDPT,
                            flags,
                        )?),
                    );
                }
                Ok(())
            }
            None => {
                self.walk_pml4(vaddr, flags)?;
                self.walk_pdpt(vaddr, flags)
            }
        }
    }

    fn walk_pd(&mut self, vaddr: VirtualAddress, flags: u64) -> Result<(), Error> {
        match &mut self.pd {
            Some(pd) => {
                unsafe {
                    let pd_ptr = addr_of_mut!(*pd);
                    self.pd = Some(
                        &mut *((*pd_ptr).get_or_map_table(
                            vaddr,
                            page_table::PageTableLevel::PD,
                            flags,
                        )?),
                    );
                }
                Ok(())
            }
            None => {
                self.walk_pdpt(vaddr, flags)?;
                self.walk_pd(vaddr, flags)
            }
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct PageMap {
    cr3: u64,
}

impl PageMap {
    pub fn try_new() -> Result<Self, Error> {
        Ok(PageMap {
            cr3: PHYSICAL_FRAME_ALLOCATOR.lock().allocate()?.bits() as u64,
        })
    }
    pub fn from_cr3(mut cr3: u64) -> Result<Self, Error> {
        // clear the PCID bits
        cr3 &= !0xFFF;

        if ArchApi::validate_paddr(cr3 as usize) == false {
            Err(Error::InvalidAddress)
        } else {
            Ok(PageMap { cr3: cr3 })
        }
    }
    pub fn get_pml4_paddr(&self) -> PhysicalAddress {
        PhysicalAddress::from((self.cr3 & !0xFFF) as usize)
    }
    pub fn get_pcid(&self) -> u16 {
        (self.cr3 & 0xFFF) as u16
    }
    pub fn set_pcid(&mut self, pcid: u16) -> Result<(), Error> {
        if self.get_pcid() != 0 {
            Err(Error::AlredyHasPcid)
        } else {
            self.cr3 = (self.cr3 & !0xFFF) | pcid as u64;
            Ok(())
        }
    }
    pub fn load(&self) -> Result<(), Error> {
        if self.get_pcid() != 0 {
            unsafe {
                asm! {
                    "mov cr3, {0}",
                    in(reg) self.cr3,
                }
            }
            Ok(())
        } else {
            Err(Error::InvalidPcid)
        }
    }
    fn invalidate_pcid(&self) {
        let mut pcid = [0u64; 2];
        pcid[0] = self.get_pcid() as u64;
        unsafe {
            asm! {
                "invpcid 1, [{pcid}]",
                pcid = in(reg) pcid.as_ptr(),
            }
        }
    }
    pub fn map_page(
        &mut self,
        vaddr: VirtualAddress,
        paddr: PhysicalAddress,
        flags: u64,
    ) -> Result<(), Error> {
        if vaddr.is_aligned_to(4096) == false {
            Err(Error::InvalidVAddrAlignment)
        } else if paddr.is_aligned_to(NonZeroUsize::new(4096).unwrap()) == false {
            Err(Error::InvalidPAddrAlignment)
        } else if vaddr.is_null() {
            Err(Error::InvalidAddress)
        } else {
            let mut walker = Walker::new(self);
            walker.walk_pd(vaddr, flags)?;
            walker
                .pt
                .unwrap()
                .map_page(page_table::PageSize::Standard, vaddr.pt_index(), flags)?;

            Ok(())
        }
    }
}
