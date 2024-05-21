//! MADT Parsing facilities
use crate::acpi::tables::{get_table, SDTHeader};
use core::mem;

/// The MADT
#[derive(Debug, Clone, Copy)]
pub struct Madt {
    header: SDTHeader,
    local_apic_addr: u32,
    flags: u32,
    addr: usize,
}

impl Madt {
    pub fn new(addr: usize) -> Madt {
        let header = get_table(addr, *b"APIC");
        if let Some(header) = header {
            let local_apic_addr = unsafe { *((addr + mem::size_of::<SDTHeader>()) as *const u32) };
            let flags = unsafe { *((addr + mem::size_of::<SDTHeader>() + 4) as *const u32) };
            Madt {
                header,
                local_apic_addr,
                flags,
                addr,
            }
        } else {
            panic!("Failed to validate MADT");
        }
    }

    pub fn local_apic_addr(&self) -> u32 {
        self.local_apic_addr
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn iter(&self) -> MadtIter {
        MadtIter {
            addr: self.addr + mem::size_of::<SDTHeader>() + 8, // Skip over the header, the local APIC address and flags
            offset: 0,
            len: self.header.length() as usize - mem::size_of::<SDTHeader>() - 8,
        }
    }
}

/// MADT Entry Iterator
pub struct MadtIter {
    addr: usize,
    offset: usize,
    len: usize,
}

impl Iterator for MadtIter {
    type Item = MadtEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.len {
            let header = unsafe { &*((self.addr + self.offset) as *const MadtEntryHeader) };

            let entry = match header.entry_type {
                0 => MadtEntry::ProcessorLocalApic(unsafe {
                    *((self.addr + self.offset) as *const ProcessorLocalApic)
                }),
                1 => MadtEntry::IOApic(unsafe { *((self.addr + self.offset) as *const IoApic) }),
                2 => MadtEntry::InterruptSourceOverride(unsafe {
                    *((self.addr + self.offset) as *const InterruptSourceOverride)
                }),
                3 => MadtEntry::NonMaskableInterruptSource(unsafe {
                    *((self.addr + self.offset) as *const NonMaskableInterruptSource)
                }),
                4 => MadtEntry::LocalApicNmi(unsafe {
                    *((self.addr + self.offset) as *const LocalApicNmi)
                }),
                5 => MadtEntry::LocalApicAddressOverride(unsafe {
                    *((self.addr + self.offset) as *const LocalApicAddressOverride)
                }),
                _ => MadtEntry::Unknown(header.entry_type),
            };
            self.offset += header.length as usize;
            Some(entry)
        } else {
            None
        }
    }
}

/// MADT Entries
#[derive(Debug)]
pub enum MadtEntry {
    ProcessorLocalApic(ProcessorLocalApic),
    IOApic(IoApic),
    InterruptSourceOverride(InterruptSourceOverride),
    NonMaskableInterruptSource(NonMaskableInterruptSource),
    LocalApicNmi(LocalApicNmi),
    LocalApicAddressOverride(LocalApicAddressOverride),
    Unknown(u8),
}

/// MADT Entry Header
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct MadtEntryHeader {
    entry_type: u8,
    length: u8,
}

/// Processor Local APIC Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorLocalApic {
    header: MadtEntryHeader,
    processor_id: u8,
    apic_id: u8,
    flags: u32,
}

/// IO APIC Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IoApic {
    header: MadtEntryHeader,
    io_apic_id: u8,
    reserved: u8,
    io_apic_addr: u32,
    global_system_interrupt_base: u32,
}

/// Interrupt Source Override Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptSourceOverride {
    header: MadtEntryHeader,
    bus: u8,
    source: u8,
    global_system_interrupt: u32,
    flags: u16,
}

/// Non-maskable Interrupt Source Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct NonMaskableInterruptSource {
    header: MadtEntryHeader,
    flags: u16,
    global_system_interrupt: u32,
}

/// Local APIC NMI Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct LocalApicNmi {
    header: MadtEntryHeader,
    processor_id: u8,
    flags: u16,
    local_apic_lint: u8,
}

/// Local APIC Address Override Structure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct LocalApicAddressOverride {
    header: MadtEntryHeader,
    reserved: u16,
    local_apic_address: u64,
}
