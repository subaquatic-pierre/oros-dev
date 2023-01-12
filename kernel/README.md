# orOS

Operating system written in Rust

### Usage

1. Build bootable image

```
cargo bootimage
```

Bootable image `bootimage-oros.bin` is found in `target/x86_64-oros/debug`

2. Convert bin image to VirtualBoxVDI

```
VBoxManage convertfromraw target/x86_64-oros/debug/bootimage-oros.bin bootimage-oros.vdi --format VDI
```
