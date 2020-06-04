CFLAGS = -target x86_64-unknown-windows -ffreestanding -fshort-wchar -mno-red-zone
LDFLAGS = -target x86_64-unknown-windows -nostdlib -Wl,-entry:efi_main -Wl,-subsystem:efi_application -fuse-ld=lld-link
QEMUFLAGS = -drive if=pflash,format=raw,file=build/OVMF.fd -drive format=raw,file=fat:rw:build/hdd -M accel=kvm:tcg -net none -serial stdio

build/kernel.o: src/main.c
	clang $(CFLAGS) -o build/kernel.o -c src/main.c -Iinclude


build/hdd/efi/boot/bootx64.efi: build/kernel.o
	clang $(LDFLAGS) -o build/hdd/efi/boot/bootx64.efi build/kernel.o


build/OVMF.fd:
	wget https://dl.bintray.com/no92/vineyard-binary/OVMF.fd -O build/OVMF.fd -qq


all: build/hdd/efi/boot/bootx64.efi build/OVMF.fd


run: all
	qemu-system-x86_64 $(QEMUFLAGS)
