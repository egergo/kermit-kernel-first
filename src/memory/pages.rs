
static mut FIRST_FREE_PAGE: usize = 0x800000;

pub fn alloc_page(num: usize) -> usize {
    unsafe {
        let result = FIRST_FREE_PAGE;
        FIRST_FREE_PAGE += num * 4096;
        ::mem::memset(result + 0xFFFFFFFF_80000000, 0, num * 4096);
        result
    }
}

