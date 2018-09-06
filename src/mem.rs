static mut LAST_PTR: usize = 0x10_00000;

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
}