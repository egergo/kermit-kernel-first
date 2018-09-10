global __hello_start
global __hello_end

section .rodata
__hello_start:
incbin "blobs/hello"
__hello_end:
