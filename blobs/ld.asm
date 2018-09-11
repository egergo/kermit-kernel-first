global __ld_start
global __ld_end

section .rodata
__ld_start:
incbin "blobs/ld-musl-x86_64.so.1"
__ld_end:
