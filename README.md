# riscv64i `#[no_std]` Linux user space example

This repository serves as reference to my later self as starting point for
`#[no_std]` rust code and to document some cargo configurations.

It builds a riscv64i Linux user space binary which invokes some syscalls
according to the Linux [syscall(2)
ABI](https://man7.org/linux/man-pages/man2/syscall.2.html) for riscv.

When running `cargo run` the binary will be executed using the QEMU riscv user
space emulator as configured in [.cargo/config](.cargo/config).

## Requirements
To run this example the QEMU riscv64 user space emulator (`qemu-riscv64`) is
required.
