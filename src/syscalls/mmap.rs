
pub fn mmap(addr: u64, length: u64, prot: u64, flags: u64, fd: u64, offset: u64) -> u64 {
    let prot = ProtFlags::from_bits(prot as i32).expect("unknown prot flags");
    let flags = MapFlags::from_bits(flags as i32).expect("unknown mmap flags");
    let offset = offset as usize;
    let mut addr = addr as usize;
    let length = length as usize;
    println!("Syscall: mmap addr={:x} length={:x} prot={:?} flags={:?} fd={} offset={:x}", addr, length, prot, flags, fd, offset);

    if addr == 0 {
        addr = 0x60000000;
    }

    unsafe {
        // enabled = true;
        ::mem::memset(addr, 0, length);

        let mut start = &::__busybox_start as *const _ as usize;
        let mut file_length = (&::__busybox_end as *const _ as usize) - start;

        if offset > file_length {
            panic!("Invalid offset")
            // TODO: just return
        }

        start += offset;
        file_length -= offset;

        if length < file_length {
            file_length = length;
        }

        ::mem::memcpy(addr, start, file_length);
        // enabled = false;
    }

    addr as u64
}

bitflags! {
    pub struct ProtFlags: i32 {
        const PROT_NONE = 0x00;
        const PROT_READ = 0x01;
        const PROT_WRITE = 0x02;
        const PROT_EXEC = 0x04;
    }
}

bitflags! {
    pub struct MapFlags: i32 {
        const MAP_SHARED = 0x01;
        const MAP_PRIVATE = 0x02;
        const MAP_SHARED_VALIDATE = 0x03;
        const MAP_FIXED = 0x10;
        const MAP_ANONYMOUS = 0x20;
        const MAP_HUGETLB = 0x40000;
        const MAP_UNINITIALIZED = 0x4000000;
        const MAP_FIXED_NOREPLACE = 0x100000;
        const MAP_NORESERVE = 0x4000;
    }
}
