use x86_64::VirtAddr;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::*;
use x86_64::structures::{gdt::*, tss::TaskStateSegment};

const INTERRUPT_STACK_SIZE: usize = 40960; // 10 pages for now

static BSP_INTERRUPT_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
static mut BSP_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
static mut BSP_TSS: TaskStateSegment = TaskStateSegment::new();
static mut BSP_SST: SegmentSelectorTable = SegmentSelectorTable::new();


pub fn setup_bsp_gdt_and_tss() {
        unsafe {
                BSP_TSS.privilege_stack_table[0] = VirtAddr::from_ptr(&BSP_INTERRUPT_STACK);

                BSP_SST.kernel_data = BSP_GDT.add_entry(Descriptor::kernel_data_segment());
                BSP_SST.kernel_code = BSP_GDT.add_entry(Descriptor::kernel_code_segment());
                BSP_SST.user_data = BSP_GDT.add_entry(Descriptor::user_data_segment());
                BSP_SST.user_code = BSP_GDT.add_entry(Descriptor::user_code_segment());
                BSP_SST.tss = BSP_GDT.add_entry(Descriptor::tss_segment(&BSP_TSS));

                BSP_GDT.load();
                load_tss(BSP_SST.tss);
                BSP_SST.set_segment_regs_kernel();
        }
}

#[allow(unused)]
pub struct SegmentSelectorTable {
        kernel_data: SegmentSelector,
        kernel_code: SegmentSelector,
        user_data: SegmentSelector,
        user_code: SegmentSelector,
        tss: SegmentSelector
}

impl SegmentSelectorTable {
        const fn new() -> Self {
                SegmentSelectorTable { 
                        kernel_data: SegmentSelector::NULL,
                        kernel_code: SegmentSelector::NULL,
                        user_data: SegmentSelector::NULL,
                        user_code: SegmentSelector::NULL,
                        tss: SegmentSelector::NULL
                }
        }
        fn set_segment_regs_kernel(&self) {
                unsafe {
                        CS::set_reg(self.kernel_code);
                        DS::set_reg(self.kernel_data);
                        ES::set_reg(self.kernel_data);
                        FS::set_reg(self.kernel_data);
                        GS::set_reg(self.kernel_data);
                        SS::set_reg(self.kernel_data);
                }
        }
        fn set_segment_regs_user(&self) {
                unsafe {
                        CS::set_reg(self.user_code);
                        DS::set_reg(self.user_data);
                        ES::set_reg(self.user_data);
                        FS::set_reg(self.user_data);
                        GS::set_reg(self.user_data);
                        SS::set_reg(self.user_data);
                }
        }
}

