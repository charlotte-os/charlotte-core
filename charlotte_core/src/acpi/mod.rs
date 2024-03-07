//! # ACPI Information
//! This module contains requests for information from the ACPI tables.

mod fadt;
mod madt;
mod rsdp;
mod sdt;
pub mod tables;

use crate::{bootinfo::RSDP_REQUEST, logln};
use core::fmt::Write;

use crate::acpi::rsdp::Rsdp;

use self::fadt::Fadt;
use self::madt::Madt;
use self::sdt::Sdt;

/// Stores the data for all the ACPI tables.
pub struct AcpiTables {
    rsdp: Rsdp,
    sdt: sdt::Sdt,
    madt: Madt,
    fadt: Fadt,
}

impl AcpiTables {
    /// Creates a new AcpiTables.
    pub fn new(rsdp: Rsdp, sdt: Sdt, madt: Madt, fadt: Fadt) -> Self {
        Self {
            rsdp,
            sdt,
            madt,
            fadt,
        }
    }

    pub fn rsdp(&self) -> &Rsdp {
        &self.rsdp
    }

    pub fn sdt(&self) -> &Sdt {
        &self.sdt
    }

    pub fn madt(&self) -> &Madt {
        &self.madt
    }
}

pub fn init_acpi() -> AcpiTables {
    if let Some(response) = RSDP_REQUEST.get_response() {
        let rsdp = Rsdp::new_from_address(response.address() as usize);
        let sdt = sdt::Sdt::new(&rsdp).unwrap();
        logln!("RSDP Signature: {}", rsdp.signature());
        logln!("RSDP Checksum: {}", rsdp.checksum());
        logln!("RSDP OEM ID: {}", rsdp.oem_id());
        logln!("RSDP Revision: {}", rsdp.revision());
        logln!("RSDP RSDT Address: {:#X}", rsdp.rsdt_address());
        if let Some(length) = rsdp.length() {
            logln!("RSDP Length: {}", length);
        }
        if let Some(xsdt_address) = rsdp.xsdt_address() {
            logln!("RSDP XSDT Address: {:#X}", xsdt_address);
        }
        if let Some(extended_checksum) = rsdp.extended_checksum() {
            logln!("RSDP Extended Checksum: {}", extended_checksum);
        }
        logln!("SDT Signature: {}", sdt.header().signature());
        logln!("SDT Length: {}", sdt.header().length());
        logln!("SDT Revision: {}", sdt.header().revision());
        logln!("SDT entry count: {}", sdt.n_entries());
        logln!("SDT address width: {}", sdt.addr_width());
        let madt = Madt::new(sdt.get_table(*b"APIC").unwrap());
        logln!("MADT Local APIC Address: {:#X}", madt.local_apic_addr());
        for entry in madt.iter() {
            logln!("MADT Entry: {:?}", entry);
        }
        let fadt = Fadt::new(sdt.get_table(*b"FACP").unwrap()).unwrap();
        logln!("Parsed FADT");
        AcpiTables::new(rsdp, sdt, madt, fadt)
    } else {
        panic!("Failed to obtain RSDP response.");
    }
}
