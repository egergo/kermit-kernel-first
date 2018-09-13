mod handlers;
mod idt;
#[macro_use]
mod macros;

use self::idt::HardwareIdt;
use ::memory::tables::{EntryFlags};
use ::memory::pages::{alloc_page};

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

pub static mut enabled: bool = true;

#[no_mangle]
pub extern "C" fn run_interrupt_fn(vars: &mut InteruptStack) {
    let irq = vars.number;
    // println!("IRQ starts {}", irq);
    match irq {
        3 => {}, // debug
        14 => {
            let cr2: u64;
            unsafe {
                asm!("mov $0, cr2" : "=r"(cr2) ::: "intel");
            }

            let pages = ::memory::tables::PageTable::address_to_tables(cr2);

            unsafe {
                if !enabled {
                    panic!("PF at 0x{:x} accessing 0x{:x} {:?}", vars.rip, cr2, pages);
                }
            }

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
        },
        32 => {}, // timer
        13 => panic!("GPF at 0x{:x}", vars.rip),
        0x80 => handle_syscall(vars),
        _ => panic!("Unknown int {}", irq)
    }

    // println!("IRQend {}", irq);
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Pollfd {
    pub fd: i32,
    pub events: i32,
    pub revents: i32
}

impl Pollfd {
    pub unsafe fn load_slice(fds: usize, nfds: usize) -> &'static [Self] {
        core::slice::from_raw_parts(fds as *const Pollfd, nfds)
    }
}

static mut POS: usize = 0;

fn handle_syscall(vars: &mut InteruptStack) {
    let syscall_number = vars.rax;
    // println!("Sys");

    // http://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
    match syscall_number {
        0 => {
            println!("Syscall: read fd={:x} buf={:x} count={:x}", vars.rdi, vars.rsi, vars.rdx);
            let count = vars.rdx as usize;
            let buf = vars.rsi as usize;

            unsafe {
                let mut to_read = &::__busybox_end as *const _ as usize - POS;
                if to_read > count {
                    to_read = count;
                }

                ::mem::memcpy(buf, POS, to_read);
                POS += to_read;
                vars.rax = to_read as u64;
            }

            // vars.rax = 0;
        }
        1 => {
            let fd = vars.rdi;
            let buf = vars.rsi as usize;
            let count = vars.rdx as usize;
            println!("Syscall: write fd={:x} buf={:x} count={:x}", fd, buf, count);

            unsafe {
                let sl = core::slice::from_raw_parts(buf as *const u8, count);
                let s = core::str::from_utf8_unchecked(sl);
                println!("-> {}", s);
            }
            vars.rax = count as u64;
        }
        2 => {
            println!("Syscall: open pathname={:x} flags={:x} mode={:x}", vars.rdi, vars.rsi, vars.rdx);
            unsafe {
                POS = &::__busybox_start as *const _ as usize;
            }
            vars.rax = 3;
        }
        3 => {
            println!("Syscall: close fd={:x}", vars.rdi);
            vars.rax = 0;
        }
        7 => {
            println!("Syscall: poll fds={:x} nfds={:x} timeout={:x}", vars.rdi, vars.rsi, vars.rdx);
            unsafe {
                let polls = Pollfd::load_slice(vars.rdi as usize, vars.rsi as usize);
                println!("FDS: {:?}", polls)
            }
            panic!();
            vars.rax = 0;
        },
        9 => {
            let prot = ProtFlags::from_bits(vars.rdx as i32).expect("unknown prot flags");
            let flags = MapFlags::from_bits(vars.r10 as i32).expect("unknown mmap flags");
            let fd = vars.r8;
            let offset = vars.r9 as usize;
            let mut addr = vars.rdi as usize;
            let length = vars.rsi as usize;
            println!("Syscall: mmap addr={:x} length={:x} prot={:?} flags={:?} fd={} offset={:x}", addr, length, prot, flags, fd, offset);

            if addr == 0 {
                addr = 0x60000000;
            }
            vars.rax = addr as u64;

            unsafe {
                enabled = true;
                ::mem::memset(addr, 0, length);

                let mut start = &::__busybox_start as *const _ as usize;
                let mut file_length = (&::__busybox_end as *const _ as usize) - start;

                if offset > file_length {
                    panic!("Invalid offset")
                    // TODO: just return
                }

                start += offset;
                file_length -= offset;

                if length < file_length {
                    file_length = length;
                }

                ::mem::memcpy(addr, start, file_length);
                enabled = false;
            }
        }
        10 => {
            let prot = ProtFlags::from_bits(vars.rdx as i32).expect("unknown prot flags");
            println!("Syscall: mprotect addr={:x} len={:x} prot={:?}", vars.rdi, vars.rsi, prot);
            vars.rax = 0;
        }
        14 => {
            println!("Syscall: sigprocmask how={:x} set={:x} oldset={:x}", vars.rdi, vars.rsi, vars.rdx);

            // SIG_BLOCK: 0
            // SIG_UNBLOCK: 1
            // SIG_SETMASK: 2
            // TODO: handle signals
            unsafe {
                if vars.rsi != 0 {
                    let set = *(vars.rsi as *const u32);
                    println!("Set: {:x}", set);
                }
            }

            unsafe {
                if vars.rdi != 0 {
                    *(vars.rdi as *mut u32) = 0;
                }
            }

            vars.rax = 0;
        }
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
        102 => {
            println!("Syscall: getuid");
            vars.rax = 0; // TODO: 0 causes exit(1)
        }
        104 => {
            println!("Syscall: getgid");
            vars.rax = 0; // TODO: 0 causes exit(1)
        }
        105 => {
            println!("Syscall: setuid uid={}", vars.rdi);
            vars.rax = 0;
        }
        106 => {
            println!("Syscall: setgid gid={}", vars.rdi);
            vars.rax = 0;
        }
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
        202 => {
            let addr = vars.rdi as usize;
            let futex_word = unsafe { *(addr as *const u32) };
            let op = vars.rsi;
            println!("Syscall: futex op={:x} addr={:x} futex_word={:x}", op, addr, futex_word);

            vars.rax = 2; // on FUTEX_WAKE
        }
        218 => { //
            println!("Syscall: set_tid_address tidptr={:x}", vars.rdi);
            unsafe {
                let mut asdf = vars.rdi as *mut usize;
                let val = *asdf;
                println!("TID: {:x}", val);
                // *asdf = 1usize;
            }
            vars.rax = 20; // thread id
        },
        231 => {
            println!("Syscall: exit_group code={:x}", vars.rdi);
            unsafe {
                asm!("hlt");
            }
        }
        x => {
            println!("Syscall rax={} 1={:x} 2={:x} 3={:x} 4={:x} 5={:x} 6={:x}", syscall_number, vars.rdi, vars.rsi, vars.rdx, vars.r10, vars.r8, vars.r9);
            panic!("Unhandled syscall {:x}", x);
        }
    }

    // println!("Sysend");

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

bitflags! {
    pub struct ProtFlags: i32 {
        const PROT_NONE = 0x00;
        const PROT_READ = 0x01;
        const PROT_WRITE = 0x02;
        const PROT_EXEC = 0x04;
    }
}

bitflags! {
    pub struct MapFlags: i32 {
        const MAP_SHARED = 0x01;
        const MAP_PRIVATE = 0x02;
        const MAP_SHARED_VALIDATE = 0x03;
        const MAP_FIXED = 0x10;
        const MAP_ANONYMOUS = 0x20;
        const MAP_HUGETLB = 0x40000;
        const MAP_UNINITIALIZED = 0x4000000;
        const MAP_FIXED_NOREPLACE = 0x100000;
        const MAP_NORESERVE = 0x4000;
    }
}
