//! # ACPI Information
//! This module contains requests for information from the ACPI tables.

mod rsdp;
pub mod tables;

use crate::{bootinfo::RSDP_REQUEST, logln};
use core::fmt::Write;

use crate::acpi::rsdp::Rsdp;

/// Stores the data for all the ACPI tables.
pub struct AcpiTables {
    rsdp: Rsdp,
}

impl AcpiTables {
    /// Creates a new AcpiTables.
    pub fn new(rsdp: Rsdp) -> Self {
        Self { rsdp }
    }

    pub fn rsdp(&self) -> &Rsdp {
        &self.rsdp
    }
}

pub fn init_acpi() -> AcpiTables {
    if let Some(response) = RSDP_REQUEST.get_response() {
        let rsdp = Rsdp::new_from_address(response.address() as usize);
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
        AcpiTables::new(rsdp)
    } else {
        panic!("Failed to obtain RSDP response.");
    }
}
