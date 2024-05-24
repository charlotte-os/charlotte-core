
mod page_table_entry;

use core::u8;

use page_table_entry::*;

use crate::arch::x86_64::memory::*;
use crate::memory::address::*;
use crate::memory::pmm::PHYSICAL_FRAME_ALLOCATOR;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Level {
    PML4 = 4,
    PDPT = 3,
    PD = 2,
    PT = 1,
}

#[repr(align(4096))]
struct PageTable<const LEVEL: u8> {
    table: [PageTableEntry; 512],
}

impl<const LEVEL: u8> PageTable<LEVEL> {
    fn new() -> Self {
        Self {
            table: [PageTableEntry::new(); 512],
        }
    }

    fn map_table(&mut self, index: usize, flags: u64) -> Result<PhysicalAddress, Error> {
        let table_paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate()?;
        self.table[index].map_table(table_paddr, flags)?;
        Ok(PHYSICAL_FRAME_ALLOCATOR.lock().allocate().unwrap())
    }

    fn unmap_table(&mut self, index: usize) -> Result<(), Error> {
        if self.table[index].is_present() {
            if self.table[index].is_size_bit_set() == false {
                PHYSICAL_FRAME_ALLOCATOR
                    .lock()
                    .deallocate(self.table[index].unmap()?);
                Ok(())
            } else {
                Err(Error::EntryNotTable)
            }
        } else {
            Err(Error::EntryNotPresent)
        }
    }

    fn map_page(&mut self, index: usize, flags: u64) -> Result<(), Error> {
        let page_paddr = match LEVEL {
            1 => PHYSICAL_FRAME_ALLOCATOR.lock().allocate()?,
            2 => PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(512, 512 * 4096)?,
            3 => PHYSICAL_FRAME_ALLOCATOR.lock().allocate_contiguous(512 * 512, 512 * 512 * 4096)?,
            _ => return Err(Error::OpNotSupportedAtThisLevel),
        };
        self.table[index].map_page(page_paddr, flags, LEVEL)
    }

    fn unmap_page(&mut self, index: usize) -> Result<(), Error> {
        let page_paddr = self.table[index].unmap()?;
        match LEVEL {
            1 => PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(page_paddr)?,
            2 => PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_contiguous(page_paddr, 512)?,
            3 => PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_contiguous(page_paddr, 512 * 512)?,
            _ => return Err(Error::OpNotSupportedAtThisLevel),
        }
        Ok(())
    }
}