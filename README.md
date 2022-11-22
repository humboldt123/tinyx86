# tinyx86
a work-in-progress kernel for experimenting with x86 baremetal development in Rust

## building
### dependencies
- `qemu` (system installation)
- `bootimage` (`cargo install bootimage`)
### running/emulation
`cargo run` will automatically build tinyx86, create a bootable image, and emulate it using your system's install of `qemu-system-x86_64`
