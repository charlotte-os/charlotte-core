//! ACPI tables handling

use core::str;

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
