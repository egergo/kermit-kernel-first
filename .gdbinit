target remote localhost:1234
symbol-file target/kernel.bin
# b start
b _start
# b double_fault_handler
# b proc.rs:167
# b proc.rs:313
b lib.rs:161
b *0x400348
c
layout split
