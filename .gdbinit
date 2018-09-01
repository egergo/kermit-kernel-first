target remote localhost:1234
symbol-file target/kernel.bin
b _start
b double_fault_handler
c
layout split
