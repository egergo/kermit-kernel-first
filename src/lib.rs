// bash -lc "DISPLAY=:0 make run"

// ../glibc/configure CFLAGS="-O2 -mno-red-zone -msoft-float -mno-sse -m64" --prefix=/tempglibc

#![feature(panic_handler)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(extern_prelude)]
#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
// mod pages;
pub mod gdt;
mod apic;
pub mod pit;
pub mod acpi;
pub mod acpica;
pub mod multiboot;

use core::panic::PanicInfo;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(wtf_handler);
        idt.non_maskable_interrupt.set_handler_fn(wtf_handler);
        idt.overflow.set_handler_fn(wtf_handler);
        idt.bound_range_exceeded.set_handler_fn(wtf_handler);
        idt.invalid_opcode.set_handler_fn(wtf_handler);
        idt.device_not_available.set_handler_fn(wtf_handler);
        // idt.coprocessor_segment_overrun.set_handler_fn(wtf_handler);
        idt.x87_floating_point.set_handler_fn(wtf_handler);
        idt.machine_check.set_handler_fn(wtf_handler);
        idt.simd_floating_point.set_handler_fn(wtf_handler);
        idt.virtualization.set_handler_fn(wtf_handler);

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        const PIC_BASE: usize = 32;
        idt[PIC_BASE + 0].set_handler_fn(apic::pit);
        idt[PIC_BASE + 1].set_handler_fn(apic::keyboard);
        idt[PIC_BASE + 2].set_handler_fn(apic::cascade);
        idt[PIC_BASE + 3].set_handler_fn(apic::com2);
        idt[PIC_BASE + 4].set_handler_fn(apic::com1);
        idt[PIC_BASE + 5].set_handler_fn(apic::lpt2);
        idt[PIC_BASE + 6].set_handler_fn(apic::floppy);
        idt[PIC_BASE + 7].set_handler_fn(apic::lpt1);
        idt[PIC_BASE + 8].set_handler_fn(apic::rtc);
        idt[PIC_BASE + 9].set_handler_fn(apic::pci1);
        idt[PIC_BASE + 10].set_handler_fn(apic::pci2);
        idt[PIC_BASE + 11].set_handler_fn(apic::pci3);
        idt[PIC_BASE + 12].set_handler_fn(apic::mouse);
        idt[PIC_BASE + 13].set_handler_fn(apic::fpu);
        idt[PIC_BASE + 14].set_handler_fn(apic::ata1);
        idt[PIC_BASE + 15].set_handler_fn(apic::ata2);

        idt[9].set_handler_fn(wtf_handler);

        idt
    };
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    println!("Kermit!");
    println!("");

    println!("Reading multiboot2...");
    multiboot::init(multiboot_information_address);

    println!("Setting up GDT...");
    gdt::init();
    println!("Setting up IDT...");
    IDT.load();
    // println!("int3...");
    // x86_64::instructions::int3();

    println!("Setting up ACPI...");
    acpi::init();

    println!("Setting up ACPICA...");
    acpica::init();

    println!("Setting up PIT...");
    pit::init();

    println!("Setting up PIC...");
    apic::init();


    // println!("Running stack overflow...");
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // // trigger a stack overflow
    // stack_overflow();

    println!("Detecting memory...");
    unsafe {
        let mbi = multiboot::get();
        let mm = mbi.memory_map_tag().expect("Memory map not provided by GRUB");
        for memory_area in mm.memory_areas() {
            println!("Memory: {:016x} - {:016x} ({} bytes)", memory_area.start_address(), memory_area.end_address(), memory_area.size());
            // pages::init_first_page(memory_area.start_address() / 4096);
        }
    }

    x86_64::instructions::interrupts::enable();

    let mut last_time = 0usize;
    loop {
        let current_time = ::apic::TIME.load(core::sync::atomic::Ordering::SeqCst);
        if current_time / 1_000_000_000 != last_time / 1_000_000_000 {
        // if current_time != last_time {
            last_time = current_time;
            println!("Time: {} {}", last_time / 1_000_000, ::pit::NANOSEC_PER_TICK);
        }
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
    // x86_64::instructions::hlt();
}

extern "x86-interrupt" fn wtf_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: WTF\n{:#?}", stack_frame);
    panic!();
}

#[allow(dead_code)]
extern "x86-interrupt" fn wtf_handler_code(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("EXCEPTION: WTF {}\n{:#?}", _error_code, stack_frame);
}

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: Divide by Zero\n{:#?}", stack_frame);
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC {:?}", _info);
    loop {}
}


// #[lang = "panic_fmt"]
// #[no_mangle]
// pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
//     line: u32) -> !
// {
//     println!("\n\nPANIC in {} at line {}:", file, line);
//     println!("    {}", fmt);
//     loop{}
// }
