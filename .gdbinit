target remote localhost:1234
symbol-file target/kernel.bin
b _start
b double_fault_handler
b proc.rs:175
b proc.rs:313
c
layout split
