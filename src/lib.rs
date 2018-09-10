// bash -lc "DISPLAY=:0 make run"

// ../glibc/configure CFLAGS="-O2 -mno-red-zone -msoft-float -mno-sse -m64" --prefix=/tempglibc

#![feature(panic_handler)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(extern_prelude)]
#![feature(naked_functions)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_let)]
#![feature(asm)]

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
pub mod mem;
pub mod proc;
pub mod interrupts;
pub mod elf;

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
        idt.general_protection_fault.set_handler_fn(gpf_handler);

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

        idt[0x80].set_handler_fn(handle_syscall)
            .set_present(true)
            .set_privilege_level(x86_64::PrivilegeLevel::Ring3);


        idt[9].set_handler_fn(wtf_handler);

        idt
    };
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    println!("Kermit!");
    println!("");

    println!("Reading multiboot2...");
    multiboot::init(multiboot_information_address);

    println!("Setting up GDT...");
    gdt::init();
    println!("Setting up IDT...");
    interrupts::init();
    // println!("int3...");
    x86_64::instructions::int3();

    println!("Setting up ACPI...");
    acpi::init();

    // println!("Setting up ACPICA...");
    // acpica::init();

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
    let mbi = multiboot::get();
    let mm = mbi.memory_map_tag().expect("Memory map not provided by GRUB");
    for memory_area in mm.memory_areas() {
        println!("Memory: {:016x} - {:016x} ({} bytes)", memory_area.start_address(), memory_area.end_address(), memory_area.size());
        // pages::init_first_page(memory_area.start_address() / 4096);
    }

    let p1 = proc::Proc::new();

    unsafe {
        ::proc::PROCESS_MANAGER.add_proc(p1);

        x86_64::instructions::interrupts::enable();
        // ::proc::PROCESS_MANAGER.tick();

        ::gdt::set_kernel_stack(x86_64::VirtAddr::new(p1.kstack as u64));
        proc::run_in_user_mode(process_handler1 as *const u8 as usize, p1.stack);
    }

    // p1.switch_to();

    println!("AFter switch1");

    let mut last_time = 0usize;
    loop {
        let current_time = ::apic::TIME.load(core::sync::atomic::Ordering::SeqCst);
        //if current_time / 1_000_000_000 != last_time / 1_000_000_000 {
        if current_time != last_time {
            last_time = current_time;
            println!("Time: {} {}", last_time / 1_000_000, ::pit::NANOSEC_PER_TICK);
        }
    }
}

extern "C" {
    static __hello_start: u8;
    static __hello_end: u8;
}

extern "C" fn process_handler1() {
    unsafe {
        println!("Hello World from Process1 {:x} {:x}", &__hello_start as *const _ as usize, &__hello_end as *const _ as usize);

        let prog = elf::ElfHeader::load(&__hello_start as *const _ as usize);
        // println!("Hello World from Process1 {:?}", prog);
        // println!("Hello World from Process1 {:?}", prog.program_header());
        prog.load_to_memory();

        asm!("
            push 0
            push 0
            push 0
            push 0
            push 0
            push 0
            push 0
            push 0
        " :::: "intel");
        asm!("jmp $0" :: "r"(prog.entry) :: "intel");
    }
    let mut last_time = 0usize;

    unsafe {
        asm!("
            mov rax, 1337
            int 80h
        " : : : "rax" : "intel")
    }

    unsafe {
        asm!("
            mov rax, 1337
            syscall
        " : : : "rax" : "intel")
    }

    unsafe {
        let rax: u64;

        asm!("mov $0, rax" : "=r"(rax) ::: "intel");
            // mov $1,rbx, 667
            // mov rcx, 668
            // mov rdx, 669
            // mov rsi, 670
            // mov rdi, 671
            // mov rbp, 672
            // mov r9, 709
            // mov r10, 710
            // mov r11, 711
            // mov r12, 712
            // mov r13, 713
            // mov r14, 714
            // mov r15, 715
        println!("RAX: {}", rax);
    }



    println!("After int80");
    loop {
        let my_rsp = get_rsp();
        let current_time = ::apic::TIME.load(core::sync::atomic::Ordering::SeqCst);
        if current_time / 1_000_000_000 != last_time / 1_000_000_000 {
        // if current_time != last_time {
            last_time = current_time;
            unsafe {
                println!("Time1: {} -> ss: {:x}, rsp: {:x} -> ss: {:x}, rsp: {:x}", last_time / 1_000_000, my_rsp.0, my_rsp.1, KERNEL_RSP.0, KERNEL_RSP.1);
            }
        }
    }
}

extern "C" fn process_handler2() {
    println!("Hello World from Process2");
    let mut last_time = 0usize;
    loop {
        let my_rsp = get_rsp();
        let current_time = ::apic::TIME.load(core::sync::atomic::Ordering::SeqCst);
        if current_time / 1_000_000_000 != last_time / 1_000_000_000 {
        // if current_time != last_time {
            last_time = current_time;
            unsafe {
                println!("Time2: {} -> ss: {:x}, rsp: {:x} -> ss: {:x}, rsp: {:x}", last_time / 1_000_000, my_rsp.0, my_rsp.1, KERNEL_RSP.0, KERNEL_RSP.1);
            }
        }
    }
}

pub static mut KERNEL_RSP: (u16, usize) = (0, 0);

pub fn get_rsp() -> (u16, usize) {
    let my_ss: u16;
    let my_rsp: usize;
    unsafe {
        asm!("mov $0, ss" : "=r"(my_ss) : : : "intel");
        asm!("mov $0, rsp" : "=r"(my_rsp) : : : "intel");
    }
    (my_ss, my_rsp)
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
    loop {

    }
}

#[allow(dead_code)]
extern "x86-interrupt" fn gpf_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    // loop {}
    println!("EXCEPTION: General Protkos Faliora {:x}\n{:#?}", _error_code, stack_frame);
    loop {

    }
}

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: Divide by Zero\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handle_syscall(_stack_frame: &mut ExceptionStackFrame) {
    // unsafe {
    //     let current_proc = ::proc::PROCESS_MANAGER.current_proc();
    //     // current_proc.rsp = stack_frame.stack_pointer.as_u64() as usize;
    //     asm!("mov $0, rsp" : "=r"(current_proc.rsp) : : : "intel");
    //     let p2 = current_proc.clone();
    //     println!("current_proc: {:?}", current_proc);
    //     println!("p2: {:?}", p2);
    //     ::proc::PROCESS_MANAGER.add_proc(p2);
    // }

    unsafe {
        // let mut current_proc = ::proc::PROCESS_MANAGER.current_proc();
        // asm!("mov $0, rsp" : "=r"(current_proc.rsp) : : : "intel");

        let mut p2 = ::proc::Proc::new();
        p2.rsp = p2.kstack;
        p2.rsp -= 8;
        *(p2.rsp as *mut u64) = thread_starter as *const u8 as u64;
        p2.rsp -= 8;
        *(p2.rsp as *mut u64) = 0;
        p2.rsp -= 8;
        *(p2.rsp as *mut u64) = p2.kstack as u64;
        println!("kstack: {:x}, thread_starter: {:x}", p2.kstack, thread_starter as *const u8 as u64);
        ::proc::PROCESS_MANAGER.add_proc(p2);

        // ::proc::PROCESS_MANAGER.current = 1;

        // ::proc::PROCESS_MANAGER.do_switch(1, 0);

        // ::gdt::set_kernel_stack(x86_64::VirtAddr::new(p2.kstack as u64));
        // proc::run_in_user_mode(process_handler2 as *const u8 as usize, p2.stack);
    }
}

#[naked]
fn thread_starter() {
    unsafe {
        println!("Thread starter");
        let current_proc = ::proc::PROCESS_MANAGER.current_proc();
        proc::run_in_user_mode(process_handler2 as *const u8 as usize, current_proc.stack);
    }
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
