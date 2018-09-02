use multiboot2::BootInformation;

static mut ADDR: usize = 0;

pub fn init(addr: usize) {
    unsafe {
        ADDR = addr;
    }
}

pub fn get() -> BootInformation {
    unsafe { multiboot2::load(ADDR) }
}
