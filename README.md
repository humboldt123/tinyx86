## tinyx86
a work-in-progress kernel for experimenting with x86 baremetal development in Rust

### dependencies
- `qemu` (system installation)
- `llvm-tools` (`rustup component add llvm-tools-preview`)
- `bootimage` (`cargo install bootimage`)
### building and emulation
`cargo run` will run the following tasks automatically, which you can also run individually:
- `cargo build`
- `cargo bootimage`
- `qemu-system-x86_64 target/x86_64-tinyx86/debug/tinyx86`
note: you may need to run `rustup update nightly --force` first to ensure all nightly components are properly installed

### testing
`cargo test` will initialize tinyx86's test framework and run tests.

