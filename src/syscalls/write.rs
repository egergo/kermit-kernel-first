
pub fn write(fd: u64, buf: u64, count: u64) -> u64 {
    let buf = buf as usize;
    let count = count as usize;
    println!("Syscall: write fd={:x} buf={:x} count={:x}", fd, buf, count);

    unsafe {
        let s = ::utils::zs_to_str_n(buf, count);
        println!("-> {}", s);
    }

    count as u64
}
