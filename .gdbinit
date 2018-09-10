target remote localhost:1234
symbol-file target/kernel.bin
# b start
b _start
# b double_fault_handler
# b proc.rs:175
# b proc.rs:313
b lib.rs:183
b *0x400348
c
layout split
