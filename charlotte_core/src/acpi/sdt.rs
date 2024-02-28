//! XSDT Parsing facilities

use core::fmt::Write;
use core::mem;

use crate::logln;

use super::{
    rsdp::Rsdp,
    tables::{self, SDTHeader},
};

#[derive(Copy, Clone)]
pub struct Sdt {
    header: SDTHeader,
    n_entries: usize,
    // TODO: when support for alloc is added, change this to a Vec<u64>
    sub_tables: [Option<SDTHeader>; 32],
    addr_width: usize,
}

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
            let table = Some(Self {
                header,
                n_entries,
                sub_tables: [None; 32],
                addr_width: 64,
            });
            table
                .unwrap()
                .populate_sub_tables((sdt_address as usize) + mem::size_of::<SDTHeader>());
            return table;
        }
        logln!("Found XSDT but failed to validate it");
        let sdt = tables::get_table(sdt_address as usize, *b"RSDT");
        if let Some(header) = sdt {
            let n_entries = (header.length() as usize - mem::size_of::<SDTHeader>()) / 4;
            let table = Some(Self {
                header,
                n_entries,
                sub_tables: [None; 32],
                addr_width: 32,
            });
            table
                .unwrap()
                .populate_sub_tables((sdt_address as usize) + mem::size_of::<SDTHeader>());
            return table;
        }
        logln!("Failed to validate RSDT, bad ACPI tables, backing off.");

        None
    }

    fn populate_sub_tables(&mut self, address: usize) {
        let ptrs = unsafe { core::slice::from_raw_parts(address as *const u64, self.n_entries) };
        for (i, ptr) in ptrs.iter().enumerate() {
            self.sub_tables[i] = tables::get_table_any_sig(*ptr as usize);
        }
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

    pub fn find_subtable(&self, signature: &[u8; 4]) -> Option<SDTHeader> {
        for i in 0..self.n_entries {
            if let Some(table) = self.sub_tables[i] {
                if *table.signature_bytes() == *signature {
                    return Some(table);
                }
            }
        }
        None
    }
}
