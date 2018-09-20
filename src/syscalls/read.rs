use super::POS;

pub fn read(fd: u64, buf: u64, count: u64) -> u64 {
    println!("Syscall: read fd={:x} buf={:x} count={:x}", fd, buf, count);
    let count = count as usize;
    let buf = buf as usize;

    unsafe {
        let mut to_read = &::__busybox_end as *const _ as usize - POS;
        if to_read > count {
            to_read = count;
        }

        ::mem::memcpy(buf, POS, to_read);
        POS += to_read;
        to_read as u64
    }
}
