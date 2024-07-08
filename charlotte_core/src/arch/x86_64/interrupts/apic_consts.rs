// src/arch/x86_64/interrupts/apic_consts.rs

/// Read/Write
pub const LAPIC_ID: u32 = 0x020;

/// Read only
pub const LAPIC_VERSION: u32 = 0x030;

/// Read/Write
pub const TASK_PRIORITY_TPR: u32 = 0x080;

/// Read only
pub const ARBITRATION_PRIORITY_APR: u32 = 0x090;

/// Read only
pub const PROCESSOR_PRIORITY_PPR: u32 = 0x0A0;

/// Write only
pub const EOI_REGISTER: u32 = 0x0B0;

/// Read only
pub const REMOTE_READ_RRD: u32 = 0x0C0;

/// Read/Write
pub const LOGICAL_DESTINATION: u32 = 0x0D0;

/// Read/Write
pub const DESTINATION_FORMAT: u32 = 0x0E0;

/// Read/Write
pub const SPURIOUS_INTERRUPT_VECTOR: u32 = 0x0F0;

/// Read only
pub const IN_SERVICE_ISR: u32 = 0x100;

/// Read only
pub const TRIGGER_MODE_TMR: u32 = 0x180;

/// Read only
pub const INTERRUPT_REQUEST_IRR: u32 = 0x200;

/// Read only
pub const ERROR_STATUS: u32 = 0x280;

/// Read/Write
pub const LVT_CORRECTED_MACHINE_CHECK_INTERRUPT_CMCI: u32 = 0x2F0;

/// Read/Write
pub const INTERRUPT_COMMAND_ICR: u32 = 0x300;

/// Read/Write
pub const LVT_TIMER: u32 = 0x320;

/// Read/Write
pub const LVT_THERMAL_SENSOR: u32 = 0x330;

/// Read/Write
pub const LVT_PERFORMANCE_MONITORING_COUNTERS: u32 = 0x340;

/// Read/Write
pub const LVT_LINT0: u32 = 0x350;

/// Read/Write
pub const LVT_LINT1: u32 = 0x360;

/// Read/Write
pub const LVT_ERROR: u32 = 0x370;

/// Read/Write
pub const TIMER_INIT_COUNT: u32 = 0x380;

/// Read only
pub const TIMER_CURRENT: u32 = 0x390;

/// Read/Write
pub const TIMER_DIVISOR: u32 = 0x3E0;

pub const APIC_DISABLE: u32 = 0x10000;

pub const APIC_NMI: u32 = 0x400;

pub const APIC_SW_ENABLE: u32 = 0x100;
