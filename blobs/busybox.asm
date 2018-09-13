global __busybox_start
global __busybox_end

section .rodata
__busybox_start:
incbin "blobs/busybox"
__busybox_end:
