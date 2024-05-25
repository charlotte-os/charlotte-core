use core::{mem, str};

use super::tables::get_table;
use super::sdt::Sdt;
use crate::{bootinfo::RSDP_REQUEST, logln};
use super::Rsdp;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SratHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: [u8; 4],
    creator_id: u32,
    creator_revision: u8,
    reserved: [u8; 12],
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Srat {
  header: SratHeader,
  length: u32, // set this to SratHeader.length
  addr: usize,
}

impl Srat {
    pub fn new(addr: usize) -> Option<Self> {
        let header = get_table(addr, *b"SRAT");
        if let Some(_header) = header {
            let srat = unsafe { *(addr as *const SratHeader) };
            Some(Srat {
                header: srat,
                length: srat.length,
                addr,
            })
        } else {
            None
        }
    }
    pub fn iter(&self) -> SratIter {
        SratIter {
            addr: self.addr + mem::size_of::<SratHeader>(),
            length: self.header.length as u32 - mem::size_of::<SratHeader>() as u32,
            offset: 0,
        }
    }
}

impl Iterator for SratIter {
    type Item = SratEntry;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.length as usize {
        let next_type =  unsafe {*((self.addr+self.offset) as *const u8)};
        let mut new_offset: usize = 0;
        let entry = match next_type {
            0 => {
                new_offset= mem::size_of::<ProcessorLocalApic>();
                
                SratEntry::ProcessorLocalApic(unsafe {
                    *((self.addr + self.offset) as *const ProcessorLocalApic)
                })
            }
            
            1 => {
                new_offset= mem::size_of::<MemoryAffinityStructure>();
                
                SratEntry::MemoryAffinityStructure(unsafe {
                    *((self.addr + self.offset) as *const MemoryAffinityStructure)
                })
            }
            
            2 => {
                new_offset= mem::size_of::<ProcessorLocalApicX2>();
                
                SratEntry::ProcessorLocalApicX2(unsafe {
                    *((self.addr + self.offset) as *const ProcessorLocalApicX2)
                })
            }
            
            _=> SratEntry::Unknown(next_type)
        };
        
        self.offset += new_offset;
        Some(entry)
        } else {
            None
        }
        
}
}
pub struct SratIter {
    addr: usize,
    length: u32,
    offset: usize,
}


#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct SratEntryHeader {
    entry_type: u8,
    length: u8,
}


#[derive(Debug)]
pub enum SratEntry {
    ProcessorLocalApic(ProcessorLocalApic),
    MemoryAffinityStructure(MemoryAffinityStructure),
    ProcessorLocalApicX2(ProcessorLocalApicX2),
    Unknown(u8),
}


#[derive(Copy, Clone, Debug)]
struct ProcessorLocalApic {
    entry_type: u8,
    length: u8,
    p_domain: u8,
    apic_id: u8,
    flags: u32,
    local_sapic_eid: u8,
    p_domain2: [u8; 24],
    clock_domain: u32,
}


#[derive(Copy, Clone, Debug)]
struct MemoryAffinityStructure {
    entry_type: u8,
    length: u8,
    p_domain: u8,
    reserved: u16,
    base_addr_low: u32,
    base_addr_high: u32,
    length_low: u32,
    length_high: u32,
    reserved_2: u32,
    flags: u32,
    reserved_3: u64,
}

#[derive(Copy, Clone, Debug)]
struct ProcessorLocalApicX2 {
    entry_type: u8,
    length: u8,
    reserved: u16,
    p_domain: u32,
    apic_id_x2: u32,
    flags: u32,
    clock_domain: u32,
    reserved_2: u32,
}