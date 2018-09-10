macro_rules! push_all_registers {
    () => {
        asm!("
            push rdi
            push rsi
            push rdx
            push rcx
            push rax
            push r8
            push r9
            push r10
            push r11
        " ::: "rsp" : "intel", "volatile");
        // Preserved:
        // push rbx
        // push rbp
        // push r12
        // push r13
        // push r14
        // push r15
    }
}

macro_rules! pop_all_registers {
    () => {
        // Preserved:
        // pop r15
        // pop r14
        // pop r13
        // pop r12
        // pop rbp
        // pop rbx
        asm!("
            pop r11
            pop r10
            pop r9
            pop r8
            pop rax
            pop rcx
            pop rdx
            pop rsi
            pop rdi
        " :::: "intel", "volatile");
    }
}