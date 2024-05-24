use core::{mem, str};

use super::tables::get_table;
use super::sdt::Sdt;
use crate::bootinfo::RSDP_REQUEST;
use super::Rsdp;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Srat {
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

impl Srat {
    pub fn new(addr: usize) -> Option<Self> {
        let header = get_table(addr, *b"SRAT");
        if let Some(_header) = header {
            let srat = unsafe { &*(addr as *const Srat) };
            Some(*srat)
        } else {
            None
        }
    }

    pub fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn oem_id(&self) -> &str {
        str::from_utf8(&self.oem_id).unwrap()
    }

    pub fn oem_table_id(&self) -> &str {
        str::from_utf8(&self.oem_table_id).unwrap()
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }

    pub fn creator_id(&self) -> u32 {
        self.creator_id
    }

    pub fn creator_revision(&self) -> u8 {
        {
            self.creator_revision
        }
    }
    
    pub fn iter(&self) -> SratIter {
        SratIter {
            offset: mem::size_of::<Srat>(),
            addr: 0,
        }
    }
}

impl Iterator for SratIter {
    type Item = SratEntry;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(response) = RSDP_REQUEST.get_response() {
        if self.addr == 0 {
            let rsdp = Rsdp::new_from_address(response.address() as usize);
            let sdt = Sdt::new(&rsdp).unwrap();
            self.addr = Sdt::get_table(&sdt, *b"SRAT").unwrap();
        }
        
        let mut header = unsafe { &*((self.addr + self.offset) as *const SratEntryHeader) };
        let mut new_offset: usize = 0;
        let entry = match header.entry_type {
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
            
            _=> SratEntry::Unknown(header.entry_type)
        };
        
        self.offset += new_offset;
        Some(entry)
        
    } else {
        panic!("Failed to obtain SRAT entries.");
    }
}
}
pub struct SratIter {
    offset: usize,
    addr: usize,
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