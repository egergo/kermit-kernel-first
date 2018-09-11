use ::memory::pages::{alloc_page};

#[derive(Copy, Clone)]
#[repr(C, align(4096))]
pub struct PageTable(pub [PageTableEntry; 512]);

bitflags! {
    pub struct EntryFlags: usize {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const ADDRESS =         0x000FFFFF_FFFFF000usize;
        const NO_EXECUTE =      1 << 63;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PageTableEntry(pub usize);
// TODO: make sure that this is x64 code

impl PageTable {
    pub const fn new() -> Self {
        PageTable([PageTableEntry::new(); 512])
    }
    pub fn address_to_tables(addr: u64) -> (usize, usize, usize, usize, usize) {
        (
            ((addr >> (9 + 9 + 9 + 12)) & 0b1_1111_1111) as usize,
            ((addr >> (9 + 9 + 12)) & 0b1_1111_1111) as usize,
            ((addr >> (9 + 12)) & 0b1_1111_1111) as usize,
            ((addr >> 12) & 0b1_1111_1111) as usize,
            (addr & 0b1111_1111_1111) as usize,
        )
    }

    pub fn create_or_get_entry(&mut self, addr: usize) -> &mut PageTableEntry {
        let pages = Self::address_to_tables(addr as u64);

        let table4 = self;
        let table3 = {
            if !table4.0[pages.0].is_present() {
                let page = alloc_page(1);
                table4.0[pages.0].set(page, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
                println!("Created Level 3 page: {:x}", page);
            }
            unsafe { table4.0[pages.0].as_table(0xFFFFFFFF_80000000) }
        };
        let table2 = {
            if !table3.0[pages.1].is_present() {
                let page = alloc_page(1);
                table3.0[pages.1].set(page, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
                println!("Created Level 2 page: {:x}", page);
            }
            unsafe { table3.0[pages.1].as_table(0xFFFFFFFF_80000000) }
        };
        let table1 = {
            if !table2.0[pages.2].is_present() {
                let page = alloc_page(1);
                table2.0[pages.2].set(page, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
                println!("Created Level 1 page: {:x}", page);
            }
            unsafe { table2.0[pages.2].as_table(0xFFFFFFFF_80000000) }
        };
        if !table1.0[pages.3].is_present() {
            let page = alloc_page(1);
            table1.0[pages.3].set(page, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
            // println!("Created used page: {:x} -> {:x} (RIP: {:x})", page, cr2, vars.rip);
        }

        &mut table1.0[pages.3]
    }
}

impl PageTableEntry {
    pub const fn new() -> Self {
        PageTableEntry(0)
    }
    pub fn is_present(&self) -> bool {
        self.0 & EntryFlags::PRESENT.bits() != 0
    }
    pub fn set_address(&mut self, addr: usize) {
        self.0 = (self.0 & !EntryFlags::ADDRESS.bits()) | (addr & EntryFlags::ADDRESS.bits())
    }
    pub fn set(&mut self, addr: usize, flags: EntryFlags) {
        self.0 = flags.bits() | (addr & EntryFlags::ADDRESS.bits());
    }
    pub fn set_flags(&mut self, flags: EntryFlags) {
        self.0 |= flags.bits();
    }
    pub fn clear_flags(&mut self, flags: EntryFlags) {
        self.0 &= !flags.bits();
    }
    pub unsafe fn as_table(&self, offset: usize) -> &mut PageTable {
        &mut*(((self.0 & EntryFlags::ADDRESS.bits()) + offset) as *mut PageTable)
    }

    // TODO: need to clear TLB here?
    // asm!("mov cr3, $0" : : "r"(&::memory::tables::PROC_TABLE as *const _ as u64 - 0xFFFFFFFF_80000000u64) : : "intel", "volatile");
}

extern "C" {
    pub static p4_table: PageTable;
    pub static p3_511_table: PageTable;
}

pub static mut PROC_TABLE: PageTable = PageTable::new();

pub fn init() {
    unsafe {
        PROC_TABLE.0[511] = p4_table.0[511];
    }
}