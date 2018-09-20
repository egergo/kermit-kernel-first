mod read;
mod write;
mod open;
mod close;
mod mmap;
mod mprotect;
mod brk;
mod ioctl;
mod writev;
mod mincore;
mod getcwd;
mod poll;
mod arch_prctl;
mod exit_group;

use super::interrupts::InteruptStack;

static mut POS: usize = 0;

pub fn handle(vars: &mut InteruptStack) {
    let syscall_number = vars.rax;

    // http://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
    let result = match syscall_number {
        0 => read::read(vars.rdi, vars.rsi, vars.rdx),
        1 => write::write(vars.rdi, vars.rsi, vars.rdx),
        2 => open::open(vars.rdi, vars.rsi, vars.rdx),
        3 => close::close(vars.rdi),
        7 => poll::poll(vars.rdi, vars.rsi, vars.rdx),
        9 => mmap::mmap(vars.rdi, vars.rsi, vars.rdx, vars.r10, vars.r8, vars.r9),
        10 => mprotect::mprotect(vars.rdi, vars.rsi, vars.rdx),
        12 => brk::brk(vars.rdi),
        13 => {
            let sig = vars.rdi as usize;
            let act = vars.rsi as usize;
            let oact = vars.rdx as usize;
            let sigsetsize = vars.r10 as usize;
            println!("Syscall: rt_sigaction sig={:x} act={:x} oact={:x} sigsetsize={:x}", sig, act, oact, sigsetsize);
            0
            // TODO: implement signals
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

            0
        }
        16 => ioctl::ioctl(vars.rdi, vars.rsi, vars.rdx),
        20 => writev::writev(vars.rdi, vars.rsi, vars.rdx),
        27 => mincore::mincore(vars.rdi, vars.rsi, vars.rdx),
        39 => {
            println!("Syscall: getpid");
            1
        }
        63 => {
            println!("Syscall: uname name={:x}", vars.rdi);
            0
        }
        79 => getcwd::getcwd(vars.rdi, vars.rsi),
        102 => {
            println!("Syscall: getuid");
            0
        }
        104 => {
            println!("Syscall: getgid");
            0
        }
        105 => {
            println!("Syscall: setuid uid={}", vars.rdi);
            0
        }
        106 => {
            println!("Syscall: setgid gid={}", vars.rdi);
            0
        }
        110 => {
            println!("Syscall: getppid");
            0
        }
        158 => arch_prctl::arch_prctl(vars.rdi, vars.rsi),
        202 => {
            let addr = vars.rdi as usize;
            let futex_word = unsafe { *(addr as *const u32) };
            let op = vars.rsi;
            println!("Syscall: futex op={:x} addr={:x} futex_word={:x}", op, addr, futex_word);

            0 // on FUTEX_WAKE
        }
        218 => { //
            println!("Syscall: set_tid_address tidptr={:x}", vars.rdi);
            unsafe {
                let mut asdf = vars.rdi as *mut usize;
                let val = *asdf;
                println!("TID: {:x}", val);
                // *asdf = 1usize;
            }
            20 // thread id
        },
        231 => exit_group::exit_group(vars.rdi),
        _ => handle_unknown_syscall(vars)
    };

    vars.rax = result;
}

pub fn handle_unknown_syscall(vars: &mut InteruptStack) -> u64 {
    println!("Syscall rax={} 1={:x} 2={:x} 3={:x} 4={:x} 5={:x} 6={:x}", vars.rax, vars.rdi, vars.rsi, vars.rdx, vars.r10, vars.r8, vars.r9);
    panic!("Unhandled syscall {}", vars.rax);
}

pub fn init() {
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

#[naked]
fn handle_fast_syscall() -> ! {
    unsafe {
        // TODO: use gs to store a pointer to current CPU and use swapgs
        // TODO: use SCRATCH_RSP from the current CPU
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

#[no_mangle]
// TODO: this is not SMP safe, need a CPU specific value
pub static SCRATCH_RSP: u64 = 0;
