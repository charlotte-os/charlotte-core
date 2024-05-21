use core::{arch::x86_64::__cpuid_count, fmt::Write};

use crate::{
    acpi::madt::{Madt, MadtEntry, ProcessorLocalApic},
    logln,
};

const FEAT_EDX_APIC: u32 = 1 << 9;

pub fn check_apic_is_present() -> bool {
    let cpuid = unsafe { __cpuid_count(0, 0) };
    cpuid.edx & FEAT_EDX_APIC != 1
}

// TODO: Make this code more reasonable
pub fn list_apics(madt: &Madt) -> [Option<ProcessorLocalApic>; 256] {
    logln!("local_apic_addr {:X?}", madt.local_apic_addr());
    let mut list = [None; 256];
    let mut i = 0;
    for entry in madt.iter() {
        if let MadtEntry::ProcessorLocalApic(lapic) = entry {
            list[i] = Some(lapic);
        }
        i += 1
    }
    list
}
