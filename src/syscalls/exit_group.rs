pub fn exit_group(code: u64) -> u64 {
    println!("Syscall: exit_group code={:x}", code);
    unsafe {
        asm!("hlt");
    }
    unreachable!()
}
