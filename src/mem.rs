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
    pub fn strlen(ptr: usize) -> usize;
}

pub fn copy_str_safe(dst: usize, s: &str, buf_len: usize) {
    if s.len() + 1 >= buf_len {
        panic!("not enough buffer {} >= {}", s.len() + 1, buf_len);
    }

    unsafe {
        memcpy(dst, s.as_ptr() as *const _ as usize, s.len());
        *((dst + s.len()) as *mut u8) = 0;
    }
}

pub unsafe fn c_to_str(ptr: usize) -> &'static str {
    let len = strlen(ptr);
    let sl = core::slice::from_raw_parts(ptr as *const u8, len);
    core::str::from_utf8_unchecked(sl)
}
