
pub fn brk(addr: u64) -> u64 {
    let addr = addr as usize;
    println!("Syscall: brk addr={:x}", addr);
    if addr == 0 {
        return 0x10000000;
    } else {
        // unsafe { enabled = true; }
        return addr as u64;
    }
}
