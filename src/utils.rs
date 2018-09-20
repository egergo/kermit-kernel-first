
pub unsafe fn zs_to_str_n(buf: usize, count: usize) -> &'static str {
    let sl = core::slice::from_raw_parts(buf as *const u8, count);
    core::str::from_utf8_unchecked(sl)
}
