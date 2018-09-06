use ::mem::malloc;

// pub struct Context {
//     /// FX valid?
//     loadable: bool,
//     /// FX location
//     fx: usize,
//     /// Page table pointer
//     cr3: usize,
//     /// RFLAGS register
//     rflags: usize,
//     /// RBX register
//     rbx: usize,
//     /// R12 register
//     r12: usize,
//     /// R13 register
//     r13: usize,
//     /// R14 register
//     r14: usize,
//     /// R15 register
//     r15: usize,
//     /// Base pointer
//     rbp: usize,
//     /// Stack pointer
//     rsp: usize
// }

// impl Context {
//     pub fn new() -> Context {
//         Context {
//             loadable: false,
//             fx: 0,
//             // cr3: 0,
//             rflags: 0,
//             rbx: 0,
//             r12: 0,
//             r13: 0,
//             r14: 0,
//             r15: 0,
//             rbp: 0,
//             rsp: 0
//         }
//     }

//     #[cold]
//     #[inline(never)]
//     #[naked]
//     pub unsafe fn switch_to(&mut self, next: &mut Context) {
//         asm!("fxsave [$0]" : : "r"(self.fx) : "memory" : "intel", "volatile");
//         self.loadable = true;
//         if next.loadable {
//             asm!("fxrstor [$0]" : : "r"(next.fx) : "memory" : "intel", "volatile");
//         }else{
//             asm!("fninit" : : : "memory" : "intel", "volatile");
//         }

//         // asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
//         // if next.cr3 != self.cr3 {
//         //     asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
//         // }

//         asm!("pushfq ; pop $0" : "=r"(self.rflags) : : "memory" : "intel", "volatile");
//         asm!("push $0 ; popfq" : : "r"(next.rflags) : "memory" : "intel", "volatile");

//         asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
//         asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");

//         asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
//         asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");

//         asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
//         asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");

//         asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
//         asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");

//         asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
//         asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");

//         asm!("mov $0, rsp" : "=r"(self.rsp) : : "memory" : "intel", "volatile");
//         asm!("mov rsp, $0" : : "r"(next.rsp) : "memory" : "intel", "volatile");

//         asm!("mov $0, rbp" : "=r"(self.rbp) : : "memory" : "intel", "volatile");
//         asm!("mov rbp, $0" : : "r"(next.rbp) : "memory" : "intel", "volatile");
//     }

// }

#[allow(dead_code)]
#[repr(packed)]
pub struct ScratchRegisters {
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rax: usize,
}

macro_rules! scratch_push {
    () => (asm!(
        "push rax
        push rcx
        push rdx
        push rdi
        push rsi
        push r8
        push r9
        push r10
        push r11"
        : : : : "intel", "volatile"
    ));
}

macro_rules! scratch_pop {
    () => (asm!(
        "pop r11
        pop r10
        pop r9
        pop r8
        pop rsi
        pop rdi
        pop rdx
        pop rcx
        pop rax"
        : : : : "intel", "volatile"
    ));
}

pub fn proc_yield() {

}




#[derive(Debug)]
pub struct ProcessManager {
    pub processes: [Proc; 16],
    pub count: usize,
    pub current: usize
}

pub static mut PROCESS_MANAGER: ProcessManager = ProcessManager::new();

impl ProcessManager {
    pub const fn new() -> ProcessManager {
        ProcessManager {
            processes: [Proc::empty(); 16],
            count: 0,
            current: 0
        }
    }

    pub fn current_proc(&mut self) -> &mut Proc {
        &mut self.processes[self.current]
    }

    pub fn add_proc(&mut self, p: Proc) {
        self.processes[self.count] = p;
        self.count = self.count + 1;
    }

    pub fn tick(&mut self) {
        // if self.running {
        //     let cur = &mut self.processes[self.current];
        //     unsafe {
        //         asm!("mov $0, rsp" : "=r"(cur.kstack) : : : "intel");
        //     }
        // }

        let from_proc = self.current;
        let to_proc = (self.current + 1) % self.count;
        if (from_proc == to_proc) {
            return;
        }

        self.current = to_proc;
        unsafe { self.switch_to(to_proc, from_proc); }
    }

    pub unsafe fn switch_to(&mut self, to_proc: usize, from_proc: usize) {
        ::gdt::set_kernel_stack(x86_64::VirtAddr::new(self.processes[to_proc].kstack as u64));
        self.do_switch(to_proc, from_proc);

        // x86_64::instructions::interrupts::disable();

        // let ds = ::gdt::GDT.1.user_data.0;
        // let cs = ::gdt::GDT.1.user_code.0;

        // unsafe {
        //     ::gdt::set_kernel_stack(x86_64::VirtAddr::new(self.kstack as u64));

        //     asm!("mov rsp, rax" : : "{rax}"(self.kstack) : : "intel", "volatile");
        //     asm!("push rax" : : "{rax}"(ds) : : "intel", "volatile");
        //     asm!("push rax" : : "{rax}"(self.stack) : : "intel", "volatile");
        //     asm!("pushf" : : : : "intel", "volatile");
        //     asm!("push rax" : : "{rax}"(cs) : : "intel", "volatile");
        //     asm!("push rax" : : "{rax}"(self.entry) : : "intel", "volatile");
        //     asm!("iretq" : : : : "intel", "volatile");

        //     println!("WTF");
        // }
    }

    #[cold]
    #[inline(never)]
    #[naked]
    unsafe fn do_switch(&mut self, to_proc: usize, from_proc: usize) {
        asm!("
            push rbx
            push rbp
        " : : : : "intel", "volatile");

        asm!("mov rax, rsp" : "={rax}"(self.processes[from_proc].rsp) : : : "intel", "volatile");
        // println!("Switching from {:x} to {:x}", self.processes[from_proc].rsp, self.processes[to_proc].rsp);
        asm!("mov rsp, rax" : : "{rax}"(self.processes[to_proc].rsp) : : "intel", "volatile");

        asm!("
            pop rbp
            pop rbx
        " : : : "rbp", "rbx" : "intel", "volatile");
    }
}

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
struct InterruptStack {
    pub entry: u64,
    pub css: u64,
    pub flags: u64,
    pub stack: u64,
    pub dss: u64
}

impl InterruptStack {
    pub unsafe fn load(addr: usize) -> &'static mut InterruptStack {
        &mut*(addr as *mut _)
    }
}

#[derive(Copy, Debug)]
pub struct Proc {
    pub stack: usize,
    pub kstack: usize,
    pub rsp: usize,
}

impl Proc {
    pub fn new() -> Proc {
        let mut result = Proc {
            stack: malloc(4096) + 4096,
            kstack: (malloc(4096) + 4096),
            rsp: 0
        };
        println!("New Proc: stack: {:x} kstack: {:x}", result.stack, result.kstack);
        result
    }

    pub const fn empty() -> Proc {
        Proc {
            stack: 0,
            kstack: 0,
            rsp: 0
        }
    }

    pub fn schedule(&self) {

    }

    // fn prepare_kstack(&mut self, entry: usize) {
    //     let ds = ::gdt::GDT.1.user_data.0;
    //     let cs = ::gdt::GDT.1.user_code.0;

    //     unsafe {
    //         self.kstack = self.kstack - 5 * 8;
    //         let s = InterruptStack::load(self.kstack);
    //         s.entry = entry as u64;
    //         s.css = cs as u64;
    //         s.flags = 0x202;
    //         s.stack = self.stack as u64;
    //         s.dss = ds as u64;
    //     }

    //     // unsafe {
    //     //     ::gdt::set_kernel_stack(x86_64::VirtAddr::new(self.kstack as u64));

    //     //     let original_sp;
    //     //     asm!("mov rsp, rax" : : "{rax}"(self.kstack) : : "intel", "volatile");
    //     //     asm!("push rax" : : "{rax}"(ds) : : "intel", "volatile");
    //     //     asm!("push rax" : : "{rax}"(self.stack) : : "intel", "volatile");
    //     //     asm!("pushf" : : : : "intel", "volatile");
    //     //     asm!("push rax" : : "{rax}"(cs) : : "intel", "volatile");
    //     //     asm!("push rax" : : "{rax}"(self.entry) : : "intel", "volatile");
    //     // }
    // }
}

impl Clone for Proc {
    fn clone(&self) -> Proc {
        let mut made = Proc::new();
        unsafe {
            ::mem::memcpy(made.stack - 4096, self.stack - 4096, 4096);
            ::mem::memcpy(made.kstack - 4096, self.kstack - 4096, 4096);
        }
        made.rsp = self.rsp;
        made.rsp += made.kstack - self.kstack;
        made
    }
}

pub fn run_in_user_mode(entry: usize, stack: usize) -> ! {
    let ds = ::gdt::GDT.1.user_data.0;
    let cs = ::gdt::GDT.1.user_code.0;

    unsafe {
        asm!("push $0" : : "r"(ds as u64) : : "intel", "volatile");
        asm!("push $0" : : "r"(stack) : : "intel", "volatile");
        asm!("push $0" : : "r"(0x202u64) : : "intel", "volatile");
        asm!("push $0" : : "r"(cs as u64) : : "intel", "volatile");
        asm!("push $0" : : "r"(entry) : : "intel", "volatile");
        asm!("iretq" : : : : "intel", "volatile");
    }

    panic!("we did an iret");
}

