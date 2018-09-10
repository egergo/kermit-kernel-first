mod handlers;
mod idt;
#[macro_use]
mod macros;

use self::idt::HardwareIdt;

lazy_static! {
    static ref IDT: HardwareIdt = {
        HardwareIdt::new()
    };
}

pub fn init() {
    IDT.load();

    unsafe {
        let addr = handle_fast_syscall as *const u8 as u64;

        asm!("wrmsr" :: "{ecx}"(0xC0000082u32), "{edx}"(addr >> 32), "{eax}"(addr as u32) :: "intel"); // IA32_LSTAR -> syscall address
        asm!("wrmsr" :: "{eax}"(0), "{edx}"(8 | 27 << 16), "{ecx}"(0xC0000081u32) :: "intel"); // STAR -> used segments
        asm!("wrmsr" :: "{eax}"(0), "{edx}"(0), "{ecx}"(0xC0000084u32) :: "intel"); // flasgs mask
        asm!("
            rdmsr
            or eax, 1
            wrmsr
        " :: "{ecx}"(0xC0000080u32) :: "intel") // IA32_EFER
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InteruptStack {
    // pub r15: u64,
    // pub r14: u64,
    // pub r13: u64,
    // pub r12: u64,
    // pub rbp: u64,
    // pub rbx: u64,

    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,

    pub number: u64,
    pub code: u64,

    pub rip: u64,
    pub cs: u64,
    pub flags: u64,
    pub rsp: u64,
    pub ds: u64
}

#[naked]
#[no_mangle]
pub extern "C" fn handle_irq() -> ! {
    unsafe {
        push_all_registers!();

        asm!("
            mov rdi, rsp
            call run_interrupt_fn
        " :::: "intel");

        pop_all_registers!();

        // pop number and code
        asm!("add rsp, 16" :::: "intel", "volatile");

        asm!("iretq" :::: "intel", "volatile");
        unreachable!();
    }
}

#[no_mangle]
pub extern "C" fn run_interrupt_fn(vars: &mut InteruptStack) {
    // println!("{:?}", vars);

    let irq = vars.number;
    match irq {
        3 => {},
        32 => {},
        13 => panic!("GPF"),
        0x80 => handle_syscall(vars),
        _ => panic!("Unknown int {}", irq)
    }
}

fn handle_syscall(vars: &mut InteruptStack) {
    let syscall_number = vars.rax;
    println!("Syscall rax={}", syscall_number);

    unsafe {
        asm!("
            mov rax, 666
            mov rbx, 667
            mov rcx, 668
            mov rdx, 669
            mov rsi, 670
            mov rdi, 671
            mov r9, 709
            mov r10, 710
            mov r11, 711
            mov r12, 712
            mov r13, 713
            mov r14, 714
            mov r15, 715
        " ::: "rsi", "rdi", "rbx" : "intel", "volatile");
    }

    vars.rax = 17;
}

#[no_mangle]
pub static SCRATCH_RSP: u64 = 0;

// #[no_mangle]
#[naked]
fn handle_fast_syscall() -> ! {
    unsafe {
        asm!("
            mov SCRATCH_RSP, rsp
            mov rsp, TSS + 4

            push 35
            push SCRATCH_RSP
            push r11
            push 27
            push rcx

            push 1
            push 0x80

            jmp handle_irq
        " :::: "intel", "volatile");
    }
    unreachable!();
}
