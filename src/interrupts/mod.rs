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
    // println!("{:?}", vars);

    let irq = vars.number;
    match irq {
        3 => {}, // debug
        14 => {
            let cr2: u64;
            unsafe {
                asm!("mov $0, cr2" : "=r"(cr2) ::: "intel");
            }
            panic!("PF at 0x{:x} accessing 0x{:x}", vars.rip, cr2);
        },
        32 => {}, // timer
        13 => panic!("GPF at 0x{:x}", vars.rip),
        0x80 => handle_syscall(vars),
        _ => panic!("Unknown int {}", irq)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Iovec {
    pub iov_base: usize,
    pub iov_len: usize
}

impl Iovec {
    pub unsafe fn load_slice(iov: usize, iovcnt: usize) -> &'static [Iovec] {
        core::slice::from_raw_parts(iov as *const Iovec, iovcnt)
    }

    pub unsafe fn to_str<'a>(&'a self) -> &'a str {
        let sl = core::slice::from_raw_parts(self.iov_base as *const u8, self.iov_len);
        core::str::from_utf8_unchecked(sl)
    }
}

fn handle_syscall(vars: &mut InteruptStack) {
    let syscall_number = vars.rax;

    // http://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
    match syscall_number {
        16 => {
            println!("Syscall: ioctl fd={:x} request={:x} ptr={:x}", vars.rdi, vars.rsi, vars.rdx);
            match vars.rsi {
                0x5413 => { // TIOCGWINSZ
                    unsafe {
                        let mut winsize = &mut*(vars.rdx as *mut Winsize);
                        winsize.ws_row = 25;
                        winsize.ws_col = 80;
                    }
                },
                _ => panic!("Unknown ioctl")
            }
            vars.rax = 0;
        },
        20 => {
            println!("Syscall: writev fd={:x} iov={:x} iovcnt={}", vars.rdi, vars.rsi, vars.rdx);
            let mut bytes_written = 0;
            unsafe {
                let vecs = Iovec::load_slice(vars.rsi as usize, vars.rdx as usize);
                for vec in vecs {
                    bytes_written += vec.iov_len;
                    println!("-> {}", vec.to_str());
                }
            }
            vars.rax = bytes_written as u64;
        },
        158 => {
            // %rdi, %rsi, %rdx, %r10, %r8 and %r9
            println!("Syscall: arch_prctl code={:x} addr={:x}", vars.rdi, vars.rsi);
            if vars.rdi == 0x1002 { // ARCH_SET_FS
                unsafe {
                    let addr = vars.rsi;
                    asm!("wrmsr" :: "{ecx}"(0xC0000100u32), "{edx}"(addr >> 32), "{eax}"(addr as u32) :: "intel"); // FSBASE
                }
            } else {
                panic!("Unknown ARCH_SET_FS {:x}", vars.rdi);
            }
            vars.rax = 0;
        },
        218 => { //
            println!("Syscall: set_tid_address tidptr={:x}", vars.rdi);
            unsafe {
                let mut asdf = vars.rdi as *mut usize;
                *asdf = 0usize;
            }
            vars.rax = 1; // thread id
        },
        231 => {
            println!("Syscall: exit_group code={:x}", vars.rdi);
            loop {};
        }
        x => {
            println!("Syscall rax={} 1={:x} 2={:x} 3={:x}", syscall_number, vars.rdi, vars.rsi, vars.rdx);
            panic!("Unhandled syscall {:x}", x);
        }
    }

    // unsafe {
    //     asm!("
    //         mov rax, 666
    //         mov rbx, 667
    //         mov rcx, 668
    //         mov rdx, 669
    //         mov rsi, 670
    //         mov rdi, 671
    //         mov r9, 709
    //         mov r10, 710
    //         mov r11, 711
    //         mov r12, 712
    //         mov r13, 713
    //         mov r14, 714
    //         mov r15, 715
    //     " ::: "rsi", "rdi", "rbx" : "intel", "volatile");
    // }

    // vars.rax = 17;
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
