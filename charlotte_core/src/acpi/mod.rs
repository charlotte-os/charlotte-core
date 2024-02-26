//! # ACPI Information
//! This module contains requests for information from the ACPI tables.

use crate::{bootinfo::RSDP_REQUEST, logln};
use core::fmt::Write;

pub fn init_acpi() {
    if let Some(response) = RSDP_REQUEST.get_response() {
        logln!("RSDP Address: {:x}", response.address() as usize);
    } else {
        panic!("Failed to obtain RSDP response.");
    }
}
