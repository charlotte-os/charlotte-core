//! ACPI tables handling

use core::str;

use core::fmt::Write;

use crate::logln;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
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

#[allow(unused)]
impl SDTHeader {
    pub fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }

    pub fn signature_bytes(&self) -> &[u8; 4] {
        &self.signature
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

pub fn get_table(address: usize, sig: [u8; 4]) -> Option<SDTHeader> {
    let header = unsafe { &*(address as *const SDTHeader) };
    if *header.signature_bytes() == sig {
        logln!("Found table with signature: {}", header.signature());
        if validate_checksum(unsafe {
            core::slice::from_raw_parts(address as *const u8, header.length() as usize)
        }) {
            logln!("Checksum is valid");
            return Some(*header);
        } else {
            logln!("Checksum is invalid");
        }
    }
    None
}

pub fn get_table_any_sig(address: usize) -> Option<SDTHeader> {
    let header = unsafe { &*(address as *const SDTHeader) };
    logln!("Found table with signature: {}", header.signature());
    if validate_checksum(unsafe {
        core::slice::from_raw_parts(address as *const u8, header.length() as usize)
    }) {
        logln!("Checksum is valid");
        return Some(*header);
    } else {
        logln!("Checksum is invalid");
    }
    None
}
