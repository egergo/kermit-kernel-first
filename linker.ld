ENTRY(start)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_OFFSET = 0xFFFFFFFF80100000;
KERNEL_OFFSET2 = 0xFFFFFFFF80000000;

SECTIONS {
	. = 1M;

	.boot :
	{
		/* ensure that the multiboot header is at the beginning */
		*(.multiboot_header)
	}

    .text.boot :
    {
        target/bootloader/boot.o(.text)
            . = ALIGN(4096);
        target/bootloader/long_mode_init.o(.text)
            . = ALIGN(4096);
    }

    .rodata.boot :
    {
        target/bootloader/boot.o(.rodata)
            . = ALIGN(4096);
    }

    .bss.boot :
    {
        target/bootloader/boot.o(.bss)
            . = ALIGN(4096);
    }

    __boot_end = .;
    . = KERNEL_OFFSET + __boot_end;

	.rodata : AT(ADDR(.rodata) - KERNEL_OFFSET2)
	{
		*(.rodata .rodata.*)
            . = ALIGN(4096);
	}

    .data :
    {
        *(.data .data.*)
            . = ALIGN(4096);
    }

	.text :
	{
		*(.text .text.*)
            . = ALIGN(4096);
	}

    .bss :
    {
        *(.bss .bss.*)
            . = ALIGN(4096);
    }

    . = ALIGN(4096);
    __cheap_malloc_start = .;
}
