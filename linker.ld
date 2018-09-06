ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		/* ensure that the multiboot header is at the beginning */
		*(.multiboot_header)
	}

	.rodata :
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
}
