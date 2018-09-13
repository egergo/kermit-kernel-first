pub mod auxdata;

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
const PT_DYNAMIC: u32 = 2;
const PT_GNU_STACK: u32 = 0x6474e551;
const PT_GNU_RELRO: u32 = 0x6474e552;
const PT_GNU_EH_FRAME: u32 = 0x6474e550;

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
                    ::mem::memset(header.virtual_address as usize, 0, header.memory_size as usize);
                    ::mem::memcpy(header.virtual_address as usize, addr + header.offset as usize, header.file_size as usize);
                    println!("LOAD: {:x} -> {:x} ({}b)", header.offset, header.virtual_address, header.file_size);
                },
                PT_DYNAMIC => {
                    ::mem::memset(header.virtual_address as usize, 0, header.memory_size as usize);
                    ::mem::memcpy(header.virtual_address as usize, addr + header.offset as usize, header.file_size as usize);
                    println!("DYNAMIC: {:x} -> {:x} ({}b)", header.offset, header.virtual_address, header.file_size);
                }
                PT_GNU_STACK => {},
                PT_GNU_RELRO => {},
                PT_GNU_EH_FRAME => {},
                x => {
                    panic!("Unknown p_type: 0x{:x}", x)
                }
            }
        }

        ::mem::memset(0x601140, 0, 0x1128);

        // TODO: zero bss
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ElfStack {
    pub top: usize,
    bottom: usize,

    env_count: usize,
    envs: [usize; 32],
    arg_count: usize,
    args: [usize; 32],

    aux_platform_ptr: usize,
    aux_execfn_ptr: usize,
    aux_random: usize
}

impl ElfStack {
    pub fn new(top: usize, size: usize) -> Self {
        ElfStack {
            top,
            bottom: top - size,
            env_count: 0,
            envs: [0; 32],
            arg_count: 0,
            args: [0; 32],
            aux_platform_ptr: 0,
            aux_execfn_ptr: 0,
            aux_random: 0
        }
    }

    pub fn write(&mut self, elf_header: &ElfHeader) {
        self.write_aux_strings();
        self.write_env_strings();
        self.write_arg_strings();

        self.write_padding();

        self.write_aux_table(elf_header);
        self.write_env_table();
        self.write_arg_table();
    }

    fn write_str(&mut self, s: &str) -> usize {
        let len = s.len();
        self.shift_top(len + 1);
        unsafe {
            ::mem::memcpy(self.top, s.as_ptr() as usize, len);
            *((self.top + len) as *mut u8) = 0;
        }
        self.top
    }

    fn write_padding(&mut self) {
        self.top = self.top & !0xf;
    }

    fn write_env_strings(&mut self) {
        self.envs[self.env_count] = self.write_str("CHARSET=UTF-8");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("HOME=/root");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("LOGNAME=root");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("PAGER=less");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("PATH=/bin");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("PS1=\\h:\\w\\$");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("PWD=/");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("SHELL=/bin/ash");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("TERM=linux");
        self.env_count += 1;
        self.envs[self.env_count] = self.write_str("USER=root");
        self.env_count += 1;
    }

    fn write_env_table(&mut self) {
        for i in 0..self.env_count {
            let tmp = self.envs[0];
            self.write_usize(tmp);
        }
        self.write_usize(0);
    }

    fn write_arg_strings(&mut self) {
        self.args[self.arg_count] = self.write_str("/ld/ld.so.1");
        self.arg_count += 1;
        self.args[self.arg_count] = self.write_str("/bin/echo");
        self.arg_count += 1;
        self.args[self.arg_count] = self.write_str("Hello World");
        self.arg_count += 1;
    }

    fn write_arg_table(&mut self) {
        self.write_usize(0);
        for i in (0..self.arg_count).rev() {
            let tmp = self.args[i];
            self.write_usize(tmp);
        }
        let tmp = self.arg_count;
        self.write_usize(tmp);
    }

    fn write_usize(&mut self, val: usize) {
        self.shift_top(8);
        unsafe {
            *(self.top as *mut usize) = val;
        }
    }

    fn shift_top(&mut self, size: usize) {
        self.top -= size;
        if self.top < self.bottom {
            panic!("ElfStack overflow");
        }
    }
}
