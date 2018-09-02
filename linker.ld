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
	}

    .data :
    {
        *(.data .data.*)
    }

	.text :
	{
		*(.text .text.*)
	}
}
