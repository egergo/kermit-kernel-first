
pub fn getcwd(buf: u64, size: u64) -> u64 {
    let buf = buf as usize;
    let size = size as usize;
    println!("Syscall: getcwd buf={:x} size={:x}", buf, size);
    ::mem::copy_str_safe(buf, "/", size);
    buf as u64
}
