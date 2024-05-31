mod page_table;

use super::Error;

use core::arch::asm;

use crate::arch::{Api, ArchApi};
use crate::arch::x86_64::cpu::huge_pages_supported;
use crate::memory::{address::PhysicalAddress, pmm::PHYSICAL_FRAME_ALLOCATOR};

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
}
