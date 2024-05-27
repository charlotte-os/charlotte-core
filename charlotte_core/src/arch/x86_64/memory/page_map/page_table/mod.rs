mod page_table_entry;

use page_table_entry::*;

use crate::arch::x86_64::memory::*;
use crate::memory::address::*;
use crate::memory::pmm::PHYSICAL_FRAME_ALLOCATOR;

#[derive(Debug, PartialEq, Eq)]
enum PageSize {
    Standard = 0,
    Large = 1,
    Huge = 2,
}

#[derive(Debug, PartialEq, Eq)]
enum PageTableLevel {
    PML4 = 4,
    PDPT = 3,
    PD = 2,
    PT = 1,
}

#[repr(align(4096))]
#[derive(Debug)]
struct PageTable {
    table: [PageTableEntry; 512],
}

impl PageTable {
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

    unsafe fn unmap_table(&mut self, index: usize) -> Result<(), Error> {
        if self.table[index].is_present() {
            if self.table[index].is_size_bit_set() == false {
                let table_paddr = self.table[index].unmap()?;
                PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(table_paddr)?;
                Ok(())
            } else {
                Err(Error::EntryNotTable)
            }
        } else {
            Err(Error::EntryNotPresent)
        }
    }

    fn map_page(&mut self, size: PageSize, index: usize, flags: u64) -> Result<(), Error> {
        let page_paddr = match size {
            PageSize::Standard => PHYSICAL_FRAME_ALLOCATOR.lock().allocate()?,
            PageSize::Large => PHYSICAL_FRAME_ALLOCATOR
                .lock()
                .allocate_contiguous(512, 512 * 4096)?,
            PageSize::Huge => PHYSICAL_FRAME_ALLOCATOR
                .lock()
                .allocate_contiguous(512 * 512, 512 * 512 * 4096)?,
        };
        self.table[index].map_page(page_paddr, flags, size)?;
        Ok(())
    }

    unsafe fn unmap_page(&mut self, size: PageSize, index: usize) -> Result<(), Error> {
        let page_paddr = self.table[index].unmap()?;
        match size {
            PageSize::Standard => PHYSICAL_FRAME_ALLOCATOR.lock().deallocate(page_paddr)?,
            PageSize::Large => PHYSICAL_FRAME_ALLOCATOR
                .lock()
                .deallocate_contiguous(page_paddr, 512)?,
            PageSize::Huge => PHYSICAL_FRAME_ALLOCATOR
                .lock()
                .deallocate_contiguous(page_paddr, 512 * 512)?,
        }
        Ok(())
    }
}
