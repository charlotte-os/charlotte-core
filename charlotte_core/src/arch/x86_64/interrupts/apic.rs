use core::{arch::x86_64::__cpuid_count, ptr};

use crate::{
    acpi::madt::{Madt, MadtEntry, ProcessorLocalApic},
    arch::x86_64::{read_msr, write_msr},
};

const FEAT_EDX_APIC: u32 = 1 << 9;
const APIC_BASE_MSR_BSP: u32 = 0x1B;
const APIC_BASE_MSR_ENABLE: u32 = 0x800;

pub fn check_apic_is_present() -> bool {
    let cpuid = unsafe { __cpuid_count(0, 0) };
    cpuid.edx & FEAT_EDX_APIC != 1
}

// TODO: Make this code more reasonable
#[allow(unused)]
pub fn list_apics(madt: &Madt) -> [Option<ProcessorLocalApic>; 256] {
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

pub fn write_apic_reg(madt: &Madt, offset: u32, value: u32) {
    let addr = (madt.local_apic_addr() + offset) as *mut u32;
    unsafe { ptr::write(addr, value) }
}

pub fn read_apic_reg(madt: &Madt, offset: u32) -> u32 {
    let addr = (madt.local_apic_addr() + offset) as *const u32;
    unsafe { ptr::read(addr) }
}

pub fn get_apic_base() -> usize {
    let msr = read_msr(APIC_BASE_MSR_BSP);
    (((msr.eax as u64) & 0xFFFFF0000 as u64) | (((msr.edx as u64) & 0x0F) << 32 as u64) as u64)
        as usize
}

pub fn set_apic_base(base: usize) {
    let eax = (base & 0xFFFFF0000) | APIC_BASE_MSR_ENABLE as usize;
    let edx = (base >> 32) & 0x0F;

    write_msr(APIC_BASE_MSR_BSP, eax as u32, edx as u32);
}
