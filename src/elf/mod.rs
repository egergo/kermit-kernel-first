#[derive(Debug)]
#[repr(C)]
pub struct ElfHeader {
    pub ident_magic: [u8; 4],
    pub ident_class: u8, // 1 - x86, 2 - 864
    pub ident_data: u8, // 1 - little endian, 2 - big endian
    pub ident_version: u8,
    pub ident_osabi: u8,
    pub ident_abiversion: u8,
    pub ident_padding: [u8; 7],
    pub elf_type: u16, // 0x02 -> ET_EXEC
    pub machine: u16, // 0x3e -> x64_64
    pub version: u32,
    pub entry: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_size: u16,
    pub program_header_number_of_entries: u16,
    pub section_header_size: u16,
    pub section_header_number_of_entries: u16,
    pub section_header_names_index: u16
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ElfProgramHeader {
    p_type: u32, // PT_GNU_STACK = 0x6474e551, PT_GNU_RELRO = 0x6474e552,
    flags: u32,
    offset: u64,
    virtual_address: u64,
    physical_address: u64,
    file_size: u64,
    memory_size: u64,
    align: u64
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ElfSectionHeader {
    name: u32,
    sh_type: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    align: u64,
    entry_size: u64
}

const PT_LOAD: u32 = 1;
const PT_GNU_STACK: u32 = 0x6474e551;
const PT_GNU_RELRO: u32 = 0x6474e552;

impl ElfHeader {
    pub unsafe fn load(addr: usize) -> &'static Self {
        &*(addr as *const Self)
    }

    pub unsafe fn program_header<'a>(&self) -> &'a [ElfProgramHeader] {
        let addr = (self as *const _ as usize) + (self.program_header_offset as usize);
        core::slice::from_raw_parts(addr as *const ElfProgramHeader, self.program_header_number_of_entries as usize)
    }

    pub unsafe fn section_header<'a>(&self) -> &'a [ElfSectionHeader] {
        let addr = (self as *const _ as usize) + (self.section_header_offset as usize);
        core::slice::from_raw_parts(addr as *const ElfSectionHeader, self.section_header_number_of_entries as usize)
    }

    pub unsafe fn load_to_memory(&self) {
        let addr = self as *const _ as usize;
        for header in self.program_header() {
            match header.p_type {
                PT_LOAD => {
                    ::mem::memcpy(header.virtual_address as usize, addr + header.offset as usize, header.file_size as usize);
                    println!("LOAD: {:x} -> {:x} ({}b)", header.offset, header.virtual_address, header.file_size);
                },
                PT_GNU_STACK => {},
                PT_GNU_RELRO => {},
                x => {
                    panic!("Unknown p_type: 0x{:x}", x)
                }
            }
        }

        ::mem::memset(0x601140, 0, 0x1128);

        // TODO: zero bss
    }
}
