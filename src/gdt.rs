use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static mut TSS: TaskStateSegment = TaskStateSegment::new();

#[allow(unused)] const GDT_DPL0: u64 = 0 << 45;
#[allow(unused)] const GDT_DPL3: u64 = 3 << 45;
#[allow(unused)] const GDT_PRESENT: u64 = 1 << 47;
#[allow(unused)] const GDT_LONG_MODE: u64 = 1 << 53;
#[allow(unused)] const GDT_CODE_READABLE: u64 = 1 << 41;
#[allow(unused)] const GDT_DATA_WRITABLE: u64 = 1 << 41;
#[allow(unused)] const GDT_TYPE_SYSTEM: u64 = 0 << 44;
#[allow(unused)] const GDT_TYPE_USER: u64 = 1 << 44;
#[allow(unused)] const GDT_EXECUTABLE: u64 = 1 << 43;
// const GDT

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code = gdt.add_entry(Descriptor::UserSegment(GDT_PRESENT | GDT_TYPE_USER | GDT_DPL0 | GDT_CODE_READABLE | GDT_LONG_MODE | GDT_EXECUTABLE));
        let kernel_data = gdt.add_entry(Descriptor::UserSegment(GDT_PRESENT | GDT_TYPE_USER | GDT_DPL0 | GDT_DATA_WRITABLE | GDT_LONG_MODE));
        let mut user_code = gdt.add_entry(Descriptor::UserSegment(GDT_PRESENT | GDT_TYPE_USER | GDT_DPL3 | GDT_CODE_READABLE | GDT_LONG_MODE | GDT_EXECUTABLE));
        let mut user_data = gdt.add_entry(Descriptor::UserSegment(GDT_PRESENT | GDT_TYPE_USER | GDT_DPL3 | GDT_DATA_WRITABLE | GDT_LONG_MODE));

        user_code.0 = user_code.0 | 3;
        user_data.0 = user_data.0 | 3;

        // let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        // let user_code = gdt.add_entry({
        //     let mut desc = Descriptor::kernel_code_segment();
        //     if let Descriptor::UserSegment(ref mut value) = desc {
        //         *value = *value | 3 << 45 | 1 << 41;
        //         println!("User Seg {:x}", *value);
        //         // *value = *value | 1 << 42
        //     }
        //     desc
        // });
        // let user_data = gdt.add_entry(Descriptor::UserSegment(GDT_PRESENT | GDT_TYPE_USER | 3 << 45 | 1 << 44 | 1 << 47 | 1 << 41));
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(unsafe { &TSS }));
        // println!("GDT: code_selector={:?}, tss_selector={:?}", code_selector, tss_selector);
        (gdt, Selectors { kernel_code, kernel_data, user_code, user_data, tss_selector })
    };
}

pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss_selector: SegmentSelector
}

pub fn init() {
    use x86_64::instructions::segmentation::*;
    use x86_64::instructions::tables::load_tss;

    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
    }

    GDT.0.load();

    unsafe {
        set_cs(GDT.1.kernel_code);
        load_ss(GDT.1.kernel_data);
        load_ds(GDT.1.kernel_data);
        load_es(GDT.1.kernel_data);
        load_fs(GDT.1.kernel_data);
        load_gs(GDT.1.kernel_data);
        load_tss(GDT.1.tss_selector);
    }
}

pub fn set_kernel_stack(ptr: VirtAddr) {
    unsafe {
        TSS.privilege_stack_table[0] = ptr;
    }
}
