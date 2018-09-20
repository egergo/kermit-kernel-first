pub fn writev(fd: u64, iov: u64, iovcnt: u64) -> u64 {
    println!("Syscall: writev fd={:x} iov={:x} iovcnt={}", fd, iov, iovcnt);
    let mut bytes_written = 0;
    unsafe {
        let vecs = Iovec::load_slice(iov as usize, iovcnt as usize);
        for vec in vecs {
            bytes_written += vec.iov_len;
            println!("-> {}", vec.to_str());
        }
    }

    bytes_written as u64
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Iovec {
    pub iov_base: usize,
    pub iov_len: usize
}

impl Iovec {
    pub unsafe fn load_slice(iov: usize, iovcnt: usize) -> &'static [Iovec] {
        core::slice::from_raw_parts(iov as *const Iovec, iovcnt)
    }

    pub unsafe fn to_str<'a>(&'a self) -> &'a str {
        let sl = core::slice::from_raw_parts(self.iov_base as *const u8, self.iov_len);
        core::str::from_utf8_unchecked(sl)
    }
}
