use crate::logln;

use super::tables::{get_table, SDTHeader};
use core::{fmt::Write, mem, usize};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Srat {
    header: SDTHeader,
    length: u32, // set this to SDTHeader.length
    addr: usize,
}

impl Srat {
    pub fn new(addr: usize) -> Option<Self> {
        let header = get_table(addr, *b"SRAT");
        if let Some(header) = header {
            Some(Srat {
                header,
                length: header.length() + 32,
                addr,
            })
        } else {
            None
        }
    }

    pub fn header(&self) -> SDTHeader {
        self.header
    }

    pub fn iter(&self) -> SratIter {
        SratIter {
            addr: self.addr,
            length: self.length - 0x30,
            offset: 0x30,
        }
    }
}

impl Iterator for SratIter {
    type Item = SratEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < (self.length - 1) as usize {
            let next_type = unsafe { *((self.addr + self.offset) as *const u8) };
            let mut next_len: usize = 1;
            logln!("{:X}", next_type);
            let entry = match next_type {
                0 => {
                    next_len = mem::size_of::<ProcessorLocalApic>();
                    SratEntry::ProcessorLocalApic(unsafe {
                        *((self.addr + self.offset) as *const ProcessorLocalApic)
                    })
                }
                1 => {
                    next_len = mem::size_of::<MemoryAffinityStructure>();
                    SratEntry::MemoryAffinityStructure(unsafe {
                        *((self.addr + self.offset) as *const MemoryAffinityStructure)
                    })
                }
                2 => {
                    next_len = mem::size_of::<ProcessorLocalApicX2>();
                    SratEntry::ProcessorLocalApicX2(unsafe {
                        *((self.addr + self.offset) as *const ProcessorLocalApicX2)
                    })
                }
                _ => SratEntry::Unknown(next_type),
            };
            self.offset += next_len;
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

#[derive(Debug)]
pub enum SratEntry {
    #[allow(dead_code)]
    ProcessorLocalApic(ProcessorLocalApic),
    #[allow(dead_code)]
    MemoryAffinityStructure(MemoryAffinityStructure),
    #[allow(dead_code)]
    ProcessorLocalApicX2(ProcessorLocalApicX2),
    #[allow(dead_code)]
    Unknown(u8),
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct ProcessorLocalApic {
    // Changed to pub
    entry_type: u8,
    length: u8,
    p_domain: u8,
    apic_id: u8,
    flags: u32,
    local_sapic_eid: u8,
    hi_dm: [u8; 3],
    clock_domain: u32,
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct MemoryAffinityStructure {
    // Changed to pub
    entry_type: u8,
    length: u8,
    p_domain: u8,
    reserved: [u8; 2],
    base_addr_low: u32,
    base_addr_high: u32,
    length_low: u32,
    length_high: u32,
    reserved_2: [u8; 4],
    flags: u32,
    reserved_3: [u8; 8],
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct ProcessorLocalApicX2 {
    // Changed to pub
    entry_type: u8,
    length: u8,
    reserved: [u8; 2],
    p_domain: u32,
    apic_id_x2: u32,
    flags: u32,
    clock_domain: u32,
    reserved_2: [u8; 4],
}
