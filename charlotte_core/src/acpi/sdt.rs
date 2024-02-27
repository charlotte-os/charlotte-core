//! XSDT Parsing facilities

use core::mem;
use core::{f32::consts::E, fmt::Write};

use crate::logln;

use super::{
    rsdp::Rsdp,
    tables::{self, SDTHeader},
};

pub struct Sdt {
    header: SDTHeader,
    n_entries: usize,
    addr_width: usize,
}

impl Sdt {
    pub fn new(rsdp: &Rsdp) -> Option<Self> {
        let sdt_address = if rsdp.xsdt_address().is_some() {
            rsdp.xsdt_address().unwrap()
        } else {
            rsdp.rsdt_address() as u64
        };
        let sdt = get_table(sdt_address as usize, *b"XSDT");
        if let Some(header) = sdt {
            let n_entries = (header.length() as usize - mem::size_of::<SDTHeader>()) / 8;
            return Some(Self {
                header,
                n_entries,
                addr_width: 64,
            });
        }
        logln!("Found XSDT but failed to validate it");
        let sdt = get_table(sdt_address as usize, *b"RSDT");
        if let Some(header) = sdt {
            let n_entries = (header.length() as usize - mem::size_of::<SDTHeader>()) / 4;
            return Some(Self {
                header,
                n_entries,
                addr_width: 32,
            });
        }
        logln!("Failed to validate RSDT");

        None
    }

    pub fn header(&self) -> &SDTHeader {
        &self.header
    }

    pub fn n_entries(&self) -> usize {
        self.n_entries
    }

    pub fn addr_width(&self) -> usize {
        self.addr_width
    }
}

fn get_table(address: usize, sig: [u8; 4]) -> Option<SDTHeader> {
    let header = unsafe { &*(address as *const SDTHeader) };
    if *header.signature_bytes() == sig {
        logln!("Found table with signature: {}", header.signature());
        if tables::validate_checksum(unsafe {
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
