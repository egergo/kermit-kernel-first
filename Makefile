# objcopy --add-section .boot=../x64/build/arch/x86_64/multiboot_header.asm.o target/x86_64-blog_os/debug/blog_os target/x86_64-blog_os/debug/blog_os2
# qemu-system-x86_64 -cdrom target/blog_os.iso

CC = /usr/bin/gcc
CFLAGS =  -std=c99 -Wall -mno-red-zone -m64 -g -nostdlib

LIBKERNEL := target/x86_64-unknown-none/debug/libblog_os.a
LIBKERNEL_SRC := Cargo.toml x86_64-unknown-none.json src/* src/**/*

BOOTLOADER_SRC_PATTERN := bootloader/%.asm
BOOTLOADER_OBJ_PATTERN := target/bootloader/%.o
BOOTLOADER_SRC := $(wildcard bootloader/*.asm)
BOOTLOADER_OBJ := $(patsubst $(BOOTLOADER_SRC_PATTERN), $(BOOTLOADER_OBJ_PATTERN), $(BOOTLOADER_SRC))

LINKER_SCRIPT := linker.ld

KERNEL := target/kernel.bin

ISO := target/blog_os.iso

LIBPATH := -L../acpica/target -L/usr/lib/gcc/x86_64-linux-gnu/7 -L/usr/lib/x86_64-linux-gnu


.PHONY: build run debug iso

run: $(ISO)
	@qemu-system-x86_64 -serial mon:stdio -m size=1024 -smp 2 -usb -device usb-kbd -vga qxl -cdrom $(ISO)

debug: $(ISO)
	@qemu-system-x86_64 -serial mon:stdio -S -s -cdrom $(ISO)
	# @qemu-system-x86_64 -d int -S -s -cdrom $(ISO)

iso: $(ISO)

$(ISO): $(KERNEL) grub.cfg
	@mkdir -p target/isofiles/boot/grub
	@cp $(KERNEL) target/isofiles/boot/kernel.bin
	@cp grub.cfg target/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) target/isofiles 2> /dev/null

$(LIBKERNEL): $(LIBKERNEL_SRC)
	RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-unknown-none

$(BOOTLOADER_OBJ_PATTERN): $(BOOTLOADER_SRC_PATTERN)
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

target/blobs/hello.o: blobs/hello.asm blobs/hello
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

target/blobs/ld.o: blobs/ld.asm blobs/ld-musl-x86_64.so.1
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

target/blobs/busybox.o: blobs/busybox.asm blobs/busybox
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

$(KERNEL): $(LINKER_SCRIPT) $(BOOTLOADER_OBJ) $(LIBKERNEL) target/blobs/hello.o target/blobs/ld.o target/blobs/busybox.o
	ld -static $(LIBPATH) -nmagic -T $(LINKER_SCRIPT) -o $(KERNEL) $(BOOTLOADER_OBJ) target/blobs/hello.o target/blobs/ld.o target/blobs/busybox.o --start-group $(LIBKERNEL) -lgcc -lacpica --end-group
