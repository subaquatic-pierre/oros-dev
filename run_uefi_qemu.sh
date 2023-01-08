#! /bin/bash

# Make iso
grub-mkrescue -o grub.iso iso

# Run qemu
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -cdrom grub.iso