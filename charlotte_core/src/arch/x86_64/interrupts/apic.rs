use core::{arch::x86_64::__cpuid_count, ptr};

use crate::arch::x86_64::cpu::asm_sti;
use crate::arch::x86_64::interrupts::apic_consts::SPURIOUS_INTERRUPT_VECTOR;
use crate::{
    acpi::madt::{Madt, MadtEntry},
    arch::x86_64::write_msr,
};

const FEAT_EDX_APIC: u32 = 0x00000200;
const APIC_BASE_MSR: u32 = 0x1B;
const APIC_BASE_MSR_ENABLE: u32 = 0x800;

pub struct Apic {
    base_phys_addr: usize,
    base_mapped_addr: Option<usize>,
}

impl Apic {
    pub fn new(madt: &Madt) -> Self {
        let addr = Self::get_apic_addr(madt);

        Apic {
            base_phys_addr: addr,
            base_mapped_addr: None,
        }
    }

    pub fn is_present() -> bool {
        let cpuid = unsafe { __cpuid_count(0, 0) };
        (cpuid.edx & FEAT_EDX_APIC) != 0
    }

    fn get_addr(&self) -> usize {
        self.base_mapped_addr.unwrap_or(self.base_phys_addr)
    }

    #[allow(unused)]
    pub fn get_apic_addr(madt: &Madt) -> usize {
        let mut addr = madt.local_apic_addr() as usize;
        let mut itr = madt.iter();
        for entry in itr {
            if let MadtEntry::LocalApicAddressOverride(addr_o) = entry {
                addr = addr_o.local_apic_address as usize;
            }
        }

        addr
    }

    pub fn write_apic_reg(&self, offset: u32, value: u32) {
        let addr = (self.get_addr() + offset as usize) as *mut u32;
        unsafe { ptr::write_volatile(addr, value) }
    }

    pub fn read_apic_reg(&self, offset: u32) -> u32 {
        let addr = (self.get_addr() + offset as usize) as *const u32;
        unsafe { ptr::read_volatile(addr) }
    }

    pub fn set_apic_base(&mut self, base: usize) {
        let eax = (base & 0xFFFFF0000) | APIC_BASE_MSR_ENABLE as usize;
        let edx = (base >> 32) & 0x0F;

        write_msr(APIC_BASE_MSR, eax as u32, edx as u32);
    }

    pub fn init(&mut self) {
        // If the apic is not present according to cpuid
        if !Apic::is_present() {
            panic!("APIC is not present, and is required!")
        }

        let base = self.get_addr();
        self.set_apic_base(base);
        // Enable spurious interrupt vector
        self.write_apic_reg(SPURIOUS_INTERRUPT_VECTOR, 0x100);
        asm_sti();
    }
}
