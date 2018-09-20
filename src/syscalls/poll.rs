
pub fn poll(fds: u64, nfds: u64, timeout: u64) -> u64 {
    println!("Syscall: poll fds={:x} nfds={:x} timeout={:x}", fds, nfds, timeout);
    unsafe {
        let polls = Pollfd::load_slice(fds as usize, nfds as usize);
        println!("FDS: {:?}", polls)
    }
    panic!();

    // 0
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Pollfd {
    pub fd: i32,
    pub events: i32,
    pub revents: i32
}

impl Pollfd {
    pub unsafe fn load_slice(fds: usize, nfds: usize) -> &'static [Self] {
        core::slice::from_raw_parts(fds as *const Pollfd, nfds)
    }
}
