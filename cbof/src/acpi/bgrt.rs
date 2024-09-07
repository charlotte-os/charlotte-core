//!  Boot Graphics Record Table (BGRT) definition

use core::str;

use super::tables::get_table;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Bgrt {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: [u8; 4],
    creator_id: u32,
    creator_revision: u8,
    version_id: u16,
    status: u8,
    image_type: u8,
    image_address: u64,
    x_offset: u32,
    y_offset: u32,
}

impl Bgrt {
    pub fn new(addr: usize) -> Option<Self> {
        let header = get_table(addr, *b"BGRT");

        if let Some(_header) = header {
            let bgrt = unsafe { &*(addr as *const Bgrt) };
            Some(*bgrt)
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

    pub fn set_version(&mut self) {
        // version must be set to 1
        self.version_id = 1
    }

    pub fn version(&self) -> u16 {
        //can be set here too, made a seperate function for debugging
        self.version_id
    }

    pub fn status(&self) -> u8 {
        self.status
    }

    pub fn image_type(&self) -> u8 {
        self.image_type
    }

    pub fn image_address(&self) -> u64 {
        self.image_address
    }
    // x offset of the image
    pub fn x_offset(&self) -> u32 {
        self.x_offset
    }

    // y offset of the image
    pub fn y_offset(&self) -> u32 {
        self.y_offset
    }

    pub fn length(&self) -> Option<u32> {
        if self.revision == 0 {
            None
        } else {
            Some(self.length)
        }
    }
}
