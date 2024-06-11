//! # ACPI Information
//! This module contains requests for information from the ACPI tables.

pub mod bgrt;
pub mod fadt;
pub mod madt;
pub mod rsdp;
pub mod sdt;
pub mod srat;
pub mod tables;

use crate::{bootinfo::RSDP_REQUEST, logln};
use core::fmt::Write;

use crate::acpi::rsdp::Rsdp;

use self::bgrt::Bgrt;
use self::fadt::Fadt;
use self::madt::Madt;
use self::sdt::Sdt;
use self::srat::Srat;

/// Stores the data for all the ACPI tables.
#[derive(Clone, Copy)]
pub struct AcpiTables {
    rsdp: Rsdp,
    sdt: Sdt,
    madt: Madt,
    #[allow(dead_code)]
    fadt: Fadt,
    bgrt: Bgrt,
    #[allow(dead_code)]
    srat: Option<Srat>,
}

impl AcpiTables {
    /// Creates a new AcpiTables.
    pub fn new(
        rsdp: Rsdp,
        sdt: Sdt,
        madt: Madt,
        fadt: Fadt,
        bgrt: Bgrt,
        srat: Option<Srat>,
    ) -> Self {
        Self {
            rsdp,
            sdt,
            madt,
            fadt,
            bgrt,
            srat,
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

    pub fn bgrt(&self) -> &Bgrt {
        &self.bgrt
    }
}

pub fn init_acpi() -> AcpiTables {
    if let Some(response) = RSDP_REQUEST.get_response() {
        let rsdp = Rsdp::new_from_address(response.address() as usize);
        let sdt = sdt::Sdt::new(&rsdp).unwrap();

        let madt = Madt::new(sdt.get_table(*b"APIC").unwrap());
        let fadt = Fadt::new(sdt.get_table(*b"FACP").unwrap()).unwrap();

        logln!("RSDP oem ID: {:?}", rsdp.oem_id());
        logln!("Parsed FADT");
        let bgrt = Bgrt::new(sdt.get_table(*b"BGRT").unwrap()).unwrap();

        if let Some(length) = bgrt.length() {
            logln!("Length: {}", length);
        } else {
            logln!("Length: None");
        }
        logln!("Parsed BGRT");
        let srat = if let Some(srat_addr) = sdt.get_table(*b"SRAT") {
            Srat::new(srat_addr)
        } else {
            None
        };
        AcpiTables::new(rsdp, sdt, madt, fadt, bgrt, srat)
    } else {
        panic!("Failed to obtain RSDP response.");
    }
}
