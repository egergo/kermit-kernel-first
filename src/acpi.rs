use core::mem;
use core::fmt;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Rsdp {
    signature: Signature8,
    checksum: u8,
    oemid: OemId,
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3]
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct SdtHeader {
    pub signature: Signature4,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: OemId,
    pub oem_table_id: OemTableId,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32
}

macro_rules! char_array_struct {
    ($name:ident, $size:expr) => {
        #[derive(Copy, Clone)]
        pub struct $name([u8; $size]);
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\"",);
                for item in self.0.iter() {
                    write!(f, "{}", *item as char);
                }
                write!(f, "\"",);
                Ok(())
            }
        }
    };
}

char_array_struct!(Signature8, 8);
char_array_struct!(Signature4, 4);
char_array_struct!(OemId, 6);
char_array_struct!(OemTableId, 8);


pub fn init() {
    let rsdp = find_rsdp().unwrap();
    println!("RSDP: {:?}", rsdp);

    let rsdt = unsafe { &*(rsdp.rsdt_address as *const SdtHeader) };
    let count = (rsdt.length as usize - mem::size_of::<SdtHeader>()) / mem::size_of::<u32>();
    println!("Found SDTs: {}", count);

    let data_address = rsdt as *const _ as usize + mem::size_of::<SdtHeader>();
    let current = data_address as *const u32;

    for x in 0..count {
        let item = unsafe { *current.offset(x as isize) };
        let sdt = unsafe { &*(item as *const SdtHeader) };
        println!("SDT: {:?}", sdt);
    }

    ()
}

pub fn find_rsdp() -> Option<Rsdp> {
    let mut addr = 0x000E0000usize;
    let found = 0;

    println!("Starting");
    loop {
        if addr % 0x1000 == 0 {
            // println!("Trying: {:x}", addr as usize);
        }

        if addr > 0x000FFFFF {
            break
        }

        let ptr = unsafe { &*(addr as *const Rsdp) };

        if &ptr.signature.0 == b"RSD PTR " {
            return Some(*ptr);
        }

        addr += 16;
    }

    println!("Found: {:x}", found as usize);

    None
}
