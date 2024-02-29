//! MADT Parsing facilities
use core::mem;

use crate::acpi::tables::{get_table, SDTHeader};

/// The MADT
pub struct Madt {
    header: SDTHeader,
    local_apic_addr: u32,
    flags: u32,
}

impl Madt {
    pub fn new(addr: usize) -> Madt {
        let header = get_table(addr, *b"APIC");
        if let Some(header) = header {
            let local_apic_addr = unsafe { *((addr + mem::size_of::<SDTHeader>()) as *const u32) };
            let flags = unsafe { *((addr + mem::size_of::<SDTHeader>() + 4) as *const u32) };
            Madt {
                header,
                local_apic_addr,
                flags,
            }
        } else {
            panic!("Failed to validate MADT");
        }
    }

    pub fn local_apic_addr(&self) -> u32 {
        self.local_apic_addr
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}
