# orOS

Operating system written in Rust

### Grub ISO

1.

```
cp target/x86_64-unknown-none/release/oros-kernel -t iso/boot/oros-kernel
grub-mkrescue -o grub.iso iso
```

### Usage

1. Build kernel image

```
cargo build --target x86_64-unknown-none
```

Bootable image `bootimage-oros.bin` is found in `target/x86_64-oros/debug`

2. Convert bin image to VirtualBoxVDI

```
VBoxManage convertfromraw target/x86_64-oros/debug/bootimage-oros.bin bootimage-oros.vdi --format VDI
```
