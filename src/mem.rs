extern {
    static __cheap_malloc_start: u8;
}
static mut LAST_PTR: usize = unsafe { &__cheap_malloc_start as *const u8 as usize };

pub fn malloc(size: u64) -> usize {
    unsafe {
        let result = LAST_PTR;
        LAST_PTR += size as usize;
        LAST_PTR += LAST_PTR % 16;
        result
    }
}
extern "C" {
    pub fn memcpy(dst: usize, src: usize, num: usize) -> usize;
    pub fn memset(ptr: usize, value: u32, num: usize) -> usize;
}