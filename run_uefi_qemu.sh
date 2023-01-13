#! /bin/bash

# Copy kernel os binary
cp target/debug/os -t iso/boot/oros-kernel

# rename kernel
mv iso/boot/os iso/boot/oros-kernel

# Make iso
grub-mkrescue -o grub.iso iso

# Run qemu
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -cdrom grub.iso