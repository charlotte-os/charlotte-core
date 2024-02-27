//! ACPI tables handling

use core::{mem, str};

use super::{rsdp, AcpiTables};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SDTHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl SDTHeader {
    pub fn new(tables: &mut AcpiTables, sig: [u8; 4]) -> Option<Self> {
        let rsdp = tables.rsdp();
        let mut address = rsdp.rsdt_address() as u64;
        // Spec says to use XSDT if its pointer is non zero
        if rsdp.xsdt_address().is_some() {
            address = rsdp.xsdt_address().unwrap();
        }

        let mut pos = address;

        let tlen = if rsdp.length().is_none() {
            panic!("RSDP length is None!")
        } else {
            rsdp.length().unwrap() as u64
        };

        while pos < address + tlen {
            let header = unsafe { &*(pos as *const SDTHeader) };
            if header.signature == sig {
                if !validate_checksum(unsafe {
                    core::slice::from_raw_parts(pos as *const u8, header.length as usize)
                }) {
                    return None;
                }
                return Some(*header);
            }
            pos += mem::size_of::<SDTHeader>() as u64;
        }
        None
    }

    pub fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn revision(&self) -> u8 {
        self.revision
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

    pub fn oem_revision(&self) -> u32 {
        self.oem_revision
    }

    pub fn creator_id(&self) -> u32 {
        self.creator_id
    }

    pub fn creator_revision(&self) -> u32 {
        self.creator_revision
    }
}

pub fn validate_checksum(data: &[u8]) -> bool {
    let mut sum: u8 = 0;
    for byte in data {
        sum = sum.wrapping_add(*byte);
    }
    sum == 0
}
