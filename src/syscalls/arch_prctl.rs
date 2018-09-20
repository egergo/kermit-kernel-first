pub fn arch_prctl(code: u64, addr: u64) -> u64 {
    println!("Syscall: arch_prctl code={:x} addr={:x}", code, addr);
    if code == 0x1002 { // ARCH_SET_FS
        unsafe {
            asm!("wrmsr" :: "{ecx}"(0xC0000100u32), "{edx}"(addr >> 32), "{eax}"(addr as u32) :: "intel"); // FSBASE
        }
    } else {
        panic!("Unknown arch_prctl code {:x}", code);
    }

    0
}
