use core::str;

const RSDP_SIGNATURE: [u8; 8] = *b"RSD PTR ";

const RDSP_V1_LEN: usize = 20; // Length of the RSDP for version 1 (20 bytes)

/// Contains information about the RSDP (Root System Description Pointer).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    // Only valid on revison 2.0 and later
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl Rsdp {
    /// Creates a new RSDP from an address
    pub fn new_from_address(address: usize) -> Self {
        let rsdp = unsafe { &*(address as *const Rsdp) };
        if rsdp.validate() {
            *rsdp
        } else {
            panic!("Failed to validate RSDP.")
        }
    }

    pub fn signature(&self) -> [u8; 8] {
        self.signature
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn oem_id(&self) -> [u8; 6] {
        self.oem_id
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }

    pub fn rsdt_address(&self) -> u32 {
        self.rsdt_address
    }

    pub fn length(&self) -> Option<u32> {
        if self.revision == 0 {
            None
        } else {
            Some(self.length)
        }
    }

    pub fn xsdt_address(&self) -> Option<u64> {
        if self.revision == 0 {
            None
        } else {
            Some(self.xsdt_address)
        }
    }

    pub fn extended_checksum(&self) -> Option<u8> {
        if self.revision == 0 {
            None
        } else {
            Some(self.extended_checksum)
        }
    }

    /// Validates the RSDP
    fn validate(&self) -> bool {
        // Ensure the signature is valid
        if self.signature != RSDP_SIGNATURE {
            panic!("Invalid RSDP signature");
        }

        // Ensure the OEM id is a valid string
        if str::from_utf8(&self.oem_id).is_err() {
            panic!("Invalid OEM ID");
        }

        // Ensure the checksum is valid
        // Length only exists on revision 2.0 and later
        let length = if self.revision == 0 {
            RDSP_V1_LEN
        } else {
            self.length as usize
        };

        let bytes =
            unsafe { core::slice::from_raw_parts(self as *const Rsdp as *const u8, length) };
        let sum = bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte));

        if sum != 0 {
            panic!("Invalid RSDP checksum");
        }

        true
    }
}
