//! # ACPI Information
//! This module contains requests for information from the ACPI tables.

use crate::acpi::rsdp::Rsdp;
use crate::bootinfo::RSDP_REQUEST;

use self::bgrt::Bgrt;
use self::fadt::Fadt;
use self::madt::Madt;
use self::sdt::Sdt;
use self::srat::Srat;

pub mod bgrt;
pub mod fadt;
pub mod madt;
pub mod rsdp;
pub mod sdt;
pub mod srat;
pub mod tables;

/// Stores the data for all the ACPI tables.
#[derive(Clone, Copy)]
pub struct AcpiTables {
    rsdp: Rsdp,
    sdt: Sdt,
    madt: Madt,
    #[allow(unused)]
    fadt: Fadt,
    bgrt: Bgrt,
    #[allow(unused)]
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

        let bgrt = Bgrt::new(sdt.get_table(*b"BGRT").unwrap()).unwrap();

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
