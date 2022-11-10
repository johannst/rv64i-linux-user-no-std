# riscv64i `#[no_std]` Linux user space example

This repository serves to my later self as reference to document some
`#[no_std]` and inline assembly features as well as cargo configurations.

It builds a `riscv64i` Linux user space binary which invokes some syscalls
according to the Linux [syscall(2)
ABI](https://man7.org/linux/man-pages/man2/syscall.2.html) for riscv.

```txt
Arch/ABI    Instruction           System  Ret  Ret  Error    Notes
                                  call #  val  val2
───────────────────────────────────────────────────────────────────
riscv       ecall                 a7      a0   a1   -

Arch/ABI      arg1  arg2  arg3  arg4  arg5  arg6  arg7  Notes
──────────────────────────────────────────────────────────────
riscv         a0    a1    a2    a3    a4    a5    -
```
The syscalls are implemented in [lib.rs](src/lib.rs).

When building, cargo runs the [build.rs](build.rs) script before building the
crate itself, which in this case generates some rust code for the syscall numbers from the riscv C headers.

The binary can be automatically run in the QEMU user space emulator by with `cargo run`, this is because the `runner` is specified in [.cargo/config](.cargo/config).

## Requirements
- `riscv64-linux-gnu-gcc` toolchain
- `rustup target add riscv64imac-unknown-none-elf` rust target
- `qemu-riscv64` QEMU userspace emulator
