pub mod tables;
pub mod pages;

use super::interrupts::InteruptStack;
use self::tables::{EntryFlags, address_to_tables};
use self::pages::{alloc_page};

pub fn handle_page_fault(_vars: &mut InteruptStack) {
    let cr2: u64;
    unsafe {
        asm!("mov $0, cr2" : "=r"(cr2) ::: "intel");
    }

    let pages = address_to_tables(cr2);

    // unsafe {
    //     if !enabled {
    //         panic!("PF at 0x{:x} accessing 0x{:x} {:?}", vars.rip, cr2, pages);
    //     }
    // }

    let table4 = unsafe { &mut ::memory::tables::PROC_TABLE };
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
    // panic!("PF at 0x{:x} accessing 0x{:x} {:?}", vars.rip, cr2, pages);
}