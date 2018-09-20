
pub fn close(fd: u64) -> u64 {
    println!("Syscall: close fd={:x}", fd);
    0
}
