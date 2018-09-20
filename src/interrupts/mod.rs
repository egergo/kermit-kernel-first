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
}

#[repr(C)]
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
    let irq = vars.number;
    // println!("IRQ starts {}", irq);
    match irq {
        3 => {}, // debug
        14 => super::memory::handle_page_fault(vars),
        32 => {}, // timer
        13 => panic!("GPF at 0x{:x}", vars.rip),
        0x80 => super::syscalls::handle(vars),
        _ => panic!("Unknown int {}", irq)
    }

    // println!("IRQend {}", irq);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Utsname {
	pub sysname: [u8; 9],
	pub nodename: [u8; 9],
	pub release: [u8; 9],
	pub version: [u8; 9],
	pub machine: [u8; 9]
}
