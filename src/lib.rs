// bootimage build --target x86_64-blog_os.json

#![feature(panic_handler)]
#![no_std]
// #![no_main]

#[macro_use]
extern crate lazy_static;
extern crate spin;

// extern crate bootloader_precompiled;
extern crate multiboot2;

use core::panic::PanicInfo;

#[macro_use]
mod console;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        let mbi = multiboot2::load(multiboot_information_address);
        let mm = mbi.memory_map_tag().expect("Memory map not provided by GRUB");
        for memory_area in mm.memory_areas() {
            println!("Memory: {:016x} - {:016x} ({} bytes)", memory_area.start_address(), memory_area.end_address(), memory_area.size());
        }
    }

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// pub fn format(args: Arguments) -> String {
//     let capacity = args.estimated_capacity();
//     let mut output = String::with_capacity(capacity);
//     output
//         .write_fmt(args)
//         .expect("a formatting trait implementation returned an error");
//     output
// }
