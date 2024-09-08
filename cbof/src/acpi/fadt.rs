//! Fixed ACPI Description Table (FADT) definition

use derive_getters::Getters;

use super::tables::{get_table, SDTHeader};

/// Address space types to make the meaning of the field more clear
#[repr(C)]
#[derive(Copy, Clone)]
pub enum AddressSpace {
    SystemMemory = 0,
    SystemIO = 1,
    PciConfig = 2,
    EmbeddedController = 3,
    SMBus = 4,
    SystemCMOS = 5,
    PciBarTarget = 6,
    IPMI = 7,
    GeneralPurposeIO = 8,
    GenericSerialBus = 9,
    PlatformCommunicationChannel = 0xA,
    FunctionalFixedHardware = 0xB,
    SystemFirmware = 0xC,
    MemoryMappedIO = 0xD,
    PlatformResource = 0xE,
    Invalid = 0xF,
}

/// Access size types to make the meaning of the field more clear
#[repr(C)]
#[derive(Copy, Clone)]
pub enum AccessSize {
    Undefined = 0,
    Byte = 1,
    Word = 2,
    DWord = 3,
    QWord = 4,
}

/// Generic address structure
/// Used to describe registers
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GenericAddress {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}

impl GenericAddress {
    pub fn address_space(&self) -> AddressSpace {
        match self.address_space {
            0 => AddressSpace::SystemMemory,
            1 => AddressSpace::SystemIO,
            2 => AddressSpace::PciConfig,
            3 => AddressSpace::EmbeddedController,
            4 => AddressSpace::SMBus,
            5 => AddressSpace::SystemCMOS,
            6 => AddressSpace::PciBarTarget,
            7 => AddressSpace::IPMI,
            8 => AddressSpace::GeneralPurposeIO,
            9 => AddressSpace::GenericSerialBus,
            0xA => AddressSpace::PlatformCommunicationChannel,
            0xB => AddressSpace::FunctionalFixedHardware,
            0xC => AddressSpace::SystemFirmware,
            0xD => AddressSpace::MemoryMappedIO,
            0xE => AddressSpace::PlatformResource,
            _ => AddressSpace::Invalid,
        }
    }

    pub fn access_size(&self) -> AccessSize {
        match self.access_size {
            1 => AccessSize::Byte,
            2 => AccessSize::Word,
            3 => AccessSize::DWord,
            4 => AccessSize::QWord,
            _ => AccessSize::Undefined,
        }
    }

    pub fn address(&self) -> u64 {
        self.address
    }

    pub fn bit_width(&self) -> u8 {
        self.bit_width
    }

    pub fn bit_offset(&self) -> u8 {
        self.bit_offset
    }
}

/// Fixed ACPI Description Table (FADT)
/// The FADT is a table that provides an ACPI-compliant OS with the information it needs to
/// enact power management related actions.
#[repr(C)]
#[derive(Copy, Clone, Getters, Debug)]
pub struct Fadt {
    header: SDTHeader,
    firmware_ctrl: u32,
    dsdt: u32,
    reserved: u8,

    preferred_pm_profile: u8,
    sci_int: u16,
    smi_cmd: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_cnt: u8,
    pm1a_evt_blk: u32,
    pm1b_evt_blk: u32,
    pm1a_cnt_blk: u32,
    pm1b_cnt_blk: u32,
    pm_timer_blk: u32,
    gpe0_blk: u32,
    gpe1_blk: u32,
    pm1_evt_len: u8,
    pm1_cnt_len: u8,
    pm_timer_len: u8,
    gpe0_blk_len: u8,
    gpe1_blk_len: u8,
    gpe1_base: u8,
    cst_cnt: u8,
    p_lvl2_lat: u16,
    p_lvl3_lat: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    century: u8,

    boot_arch_flags: u16,
    reserved2: u8,
    flags: u32,

    reset_reg: GenericAddress,
    reset_value: u8,
    reserved3: [u8; 3],

    x_firmware_ctrl: u64,
    x_dsdt: u64,

    x_pm1a_evt_blk: GenericAddress,
    x_pm1b_evt_blk: GenericAddress,
    x_pm1a_cnt_blk: GenericAddress,
    x_pm1b_cnt_blk: GenericAddress,
    x_pm_timer_blk: GenericAddress,
    x_gpe0_blk: GenericAddress,
    x_gpe1_blk: GenericAddress,
}

impl Fadt {
    pub fn new(addr: usize) -> Option<Self> {
        let header = get_table(addr, *b"FACP");

        if let Some(_header) = header {
            let fadt = unsafe { &*(addr as *const Fadt) };
            Some(*fadt)
        } else {
            None
        }
    }
}
