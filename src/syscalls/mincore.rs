
pub fn mincore(addr: u64, length: u64, vec: u64) -> u64 {
    let addr = addr as usize;
    let length = length as usize;
    let vec = vec as usize;
    println!("Syscall: mincore addr={:x} length={:x} vex={:x}", addr, length, vec);
    0
}
