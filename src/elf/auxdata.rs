
bitflags! {
    struct AuxType: usize {
        const AT_NULL = 0; /* end of vector */
        const AT_IGNORE = 1; /* entry should be ignored */
        const AT_EXECFD = 2; /* file descriptor of program */
        const AT_PHDR = 3; /* program headers for program */
        const AT_PHENT = 4; /* size of program header entry */
        const AT_PHNUM = 5; /* number of program headers */
        const AT_PAGESZ = 6; /* system page size */
        const AT_BASE = 7; /* base address of interpreter */
        const AT_FLAGS = 8; /* flags */
        const AT_ENTRY = 9; /* entry point of program */
        const AT_NOTELF = 10; /* program is not ELF */
        const AT_UID = 11; /* real uid */
        const AT_EUID = 12; /* effective uid */
        const AT_GID = 13; /* real gid */
        const AT_EGID = 14; /* effective gid */
        const AT_PLATFORM = 15; /* string identifying CPU for optimizations */
        const AT_HWCAP = 16; /* arch dependent hints at CPU capabilities */
        const AT_CLKTCK = 17; /* frequency at which times() increments */
        const AT_SECURE = 23; /* secure mode boolean */
        const AT_BASE_PLATFORM = 24; /* string identifying real platform, may constr = from AT_PLATFORM. */
        const AT_RANDOM = 25; /* address of 16 random bytes */
        const AT_HWCAP2 = 26; /* extension of AT_HWCAP */
        const AT_EXECFN = 31; /* filename of program */
    }
}

impl super::ElfStack {
    pub fn write_aux_strings(&mut self) {
        self.aux_platform_ptr = self.write_str("x86_64");
        self.aux_execfn_ptr = self.write_str("/bin/ld");
        self.shift_top(16);
        self.aux_random = self.top;
    }

    pub fn write_aux_table(&mut self, elf_header: &super::ElfHeader) {
        self.write_aux_value(AuxType::AT_NULL, 0);
        self.write_aux_value(AuxType::AT_BASE, 0);
        self.write_aux_value(AuxType::AT_FLAGS, 0);
        self.write_aux_value(AuxType::AT_HWCAP, 0);
        self.write_aux_value(AuxType::AT_PHDR, elf_header.program_header_offset as usize);
        self.write_aux_value(AuxType::AT_PHENT, elf_header.program_header_size as usize);
        self.write_aux_value(AuxType::AT_PHNUM, elf_header.program_header_number_of_entries as usize);
        self.write_aux_value(AuxType::AT_ENTRY, elf_header.entry as usize);
        let tmp = self.aux_execfn_ptr;
        self.write_aux_value(AuxType::AT_EXECFN, tmp);
        let tmp = self.aux_platform_ptr;
        self.write_aux_value(AuxType::AT_PLATFORM, tmp);
        self.write_aux_value(AuxType::AT_PAGESZ, 4096);
        self.write_aux_value(AuxType::AT_CLKTCK, 100);

        self.write_aux_value(AuxType::AT_UID, 0);
        self.write_aux_value(AuxType::AT_EUID, 0);
        self.write_aux_value(AuxType::AT_GID, 0);
        self.write_aux_value(AuxType::AT_EGID, 0);
        self.write_aux_value(AuxType::AT_SECURE, 0);

        let tmp = self.aux_random;
        self.write_aux_value(AuxType::AT_RANDOM, tmp);
    }

    fn write_aux_value(&mut self, key: AuxType, value: usize) {
        self.write_usize(value);
        self.write_usize(key.bits);
    }
}
