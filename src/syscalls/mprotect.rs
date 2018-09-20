use super::mmap::ProtFlags;

pub fn mprotect(addr: u64, len: u64, prot: u64) -> u64 {
    let prot = ProtFlags::from_bits(prot as i32).expect("unknown prot flags");
    println!("Syscall: mprotect addr={:x} len={:x} prot={:?}", addr, len, prot);
    0
}
