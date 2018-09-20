pub fn ioctl(fd: u64, request: u64, ptr: u64) -> u64 {
    println!("Syscall: ioctl fd={:x} request={:x} ptr={:x}", fd, request, ptr);
    match request {
        0x5413 => { // TIOCGWINSZ
            unsafe {
                let mut winsize = &mut*(ptr as *mut Winsize);
                winsize.ws_row = 25;
                winsize.ws_col = 80;
            }
        },
        _ => panic!("Unknown ioctl")
    }
    0
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
