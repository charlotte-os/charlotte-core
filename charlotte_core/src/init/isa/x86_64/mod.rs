mod gdt;

use core::ptr::addr_of;
use spin::Lazy;

use crate::logln;

use super::super::InitApi;




static BSP_RSP0: [u8; 4096] = [0; 4096];
static BSP_TSS: Lazy<gdt::tss::Tss> = Lazy::new(|| gdt::tss::Tss::new(addr_of!(BSP_RSP0) as u64));
static BSP_GDT: Lazy<gdt::Gdt> = Lazy::new(|| gdt::Gdt::new(&BSP_TSS));

pub struct InitApiImpl;

impl InitApi for InitApiImpl {
    fn init_system() {
        Self::init_bsp();
    }
    fn init_bsp() {
        logln!("Loading BSP GDT");
        BSP_GDT.load();
        logln!("GDT loaded");
        logln!("Reloading BSP segment registers");
        gdt::Gdt::reload_segment_regs();
        logln!("Segment registers reloaded");
        logln!("Loading BSP TSS");
        gdt::Gdt::load_tss();
        logln!("TSS loaded");
    }
    fn init_ap() {
        todo!()
    }
}