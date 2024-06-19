//! XSDT Parsing facilities

use core::fmt::Write;
use core::mem;

use crate::logln;

use super::{
    rsdp::Rsdp,
    tables::{self, SDTHeader},
};

/// An entry in the XSDT or RSDT
#[derive(Copy, Clone)]
struct SdtEntry {
    signature: [u8; 4],
    addr: usize,
}

/// The XSDT or RSDT
#[derive(Copy, Clone)]
pub struct Sdt {
    header: SDTHeader,
    n_entries: usize,
    // TODO: when support for alloc is added, change this to a Vec<u64>
    sub_tables: [Option<SdtEntry>; 32],
    addr_width: usize,
}

#[allow(unused)]
impl Sdt {
    pub fn new(rsdp: &Rsdp) -> Option<Self> {
        let sdt_address = if rsdp.xsdt_address().is_some() {
            rsdp.xsdt_address().unwrap()
        } else {
            rsdp.rsdt_address() as u64
        };
        let sdt = tables::get_table(sdt_address as usize, *b"XSDT");
        if let Some(header) = sdt {
            let n_entries = (header.length() as usize - mem::size_of::<SDTHeader>()) / 8;
            let sub_tables = populate_sub_tables(
                (sdt_address as usize) + mem::size_of::<SDTHeader>(),
                n_entries,
                64,
            );
            let table = Some(Self {
                header,
                n_entries,
                sub_tables,
                addr_width: 64,
            });
            return table;
        }
        logln!("Found XSDT but failed to validate it");
        let sdt = tables::get_table(sdt_address as usize, *b"RSDT");
        if let Some(header) = sdt {
            let n_entries = (header.length() as usize - mem::size_of::<SDTHeader>()) / 4;
            let sub_tables = populate_sub_tables(
                (sdt_address as usize) + mem::size_of::<SDTHeader>(),
                n_entries,
                32,
            );
            let table = Some(Self {
                header,
                n_entries,
                sub_tables,
                addr_width: 32,
            });
            return table;
        }
        logln!("Failed to validate RSDT, bad ACPI tables, backing off.");

        None
    }

    /// Get the address of a table from the XSDT or RSDT
    pub fn get_table(&self, signature: [u8; 4]) -> Option<usize> {    
        for i in 0..self.n_entries {
            if self.sub_tables[i].is_some() && self.sub_tables[i].unwrap().signature == signature {
                return Some(self.sub_tables[i].unwrap().addr);
            }
        }
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

/// Populates the subtables of the XSDT or RSDT
fn populate_sub_tables(
    address: usize,
    n_entries: usize,
    addr_width: usize,
) -> [Option<SdtEntry>; 32] {
    let mut sub_tables: [Option<SdtEntry>; 32] = [None; 32];
    #[allow(clippy::needless_range_loop)]
    for i in 0..n_entries {
        let mut ptr: usize = 0;
        // We need to grab each half independently since the XSDT uses 64-bit pointers
        // but the RSDT uses 32-bit pointers.

        // The XSDT addresses are also 4-byte aligned, so we can't treat its entries as u64
        // as dereferencing them would cause a misaligned access
        let ptr_low = unsafe { *((address + i * (addr_width / 8)) as *const u32) };
        ptr |= ptr_low as usize;
        if addr_width == 64 {
            let ptr_high = unsafe { *((address + i * (addr_width / 8) + 4) as *const u32) };
            ptr |= (ptr_high as usize) << 32;
        }
        let table = tables::get_table_any_sig(ptr);
        if let Some(header) = table {
            sub_tables[i] = Some(SdtEntry {
                signature: *header.signature_bytes(),
                addr: ptr,
            });
        }
    }
    sub_tables
}
