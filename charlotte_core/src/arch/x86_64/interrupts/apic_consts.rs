// src/arch/x86_64/interrupts/apic_consts.rs

#[allow(dead_code)]
pub const APIC_REG_LAPICID: u32 = 0x20;
#[allow(dead_code)]
pub const APIC_REG_LAPICVR: u32 = 0x30;
#[allow(dead_code)]
pub const APIC_REG_TPR: u32 = 0x80;
#[allow(dead_code)]
pub const APIC_REG_APR: u32 = 0x90;
#[allow(dead_code)]
pub const APIC_REG_PPR: u32 = 0xA0;
#[allow(dead_code)]
pub const APIC_REG_EOI: u32 = 0xB0;
#[allow(dead_code)]
pub const APIC_REG_RRD: u32 = 0xC0;
#[allow(dead_code)]
pub const APIC_REG_LDR: u32 = 0xD0;
#[allow(dead_code)]
pub const APIC_REG_DFR: u32 = 0xE0;
#[allow(dead_code)]
pub const APIC_REG_SIR: u32 = 0xF0;
#[allow(dead_code)]
pub const APIC_REG_ISR_START: u32 = 0x100;
#[allow(dead_code)]
pub const APIC_REG_ISR_END: u32 = 0x170;
#[allow(dead_code)]
pub const APIC_REG_TMR_START: u32 = 0x180;
#[allow(dead_code)]
pub const APIC_REG_TMR_END: u32 = 0x1F0;
#[allow(dead_code)]
pub const APIC_REG_IRR_START: u32 = 0x200;
#[allow(dead_code)]
pub const APIC_REG_IRR_END: u32 = 0x270;
#[allow(dead_code)]
pub const APIC_ERROR_STATUS_REG: u32 = 0x280;
#[allow(dead_code)]
pub const APIC_TIMER_ICR: u32 = 0x380;
#[allow(dead_code)]
pub const APIC_TIMER_CCR: u32 = 0x390;
#[allow(dead_code)]
pub const APIC_TIMER_DCR: u32 = 0x3E0;
