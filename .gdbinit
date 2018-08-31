target remote localhost:1234
symbol-file target/kernel.bin
b _start
c
layout split
