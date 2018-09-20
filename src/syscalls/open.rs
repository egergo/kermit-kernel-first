use super::POS;

pub fn open(pathname: u64, flags: u64, mode: u64) -> u64 {
    let pathname_ptr = pathname as usize;
    let pathname = unsafe { ::mem::c_to_str(pathname_ptr) };
    println!("Syscall: open pathname={} flags={:x} mode={:x}", pathname, flags, mode);

    if pathname == "/dev/tty" {
        return 0xFFFFFFFF_FFFFFFFF;
    } else {
        unsafe {
            POS = &::__busybox_start as *const _ as usize;
        }
    }

    3
}
