#![no_std]

/// Raw file descriptor.
///
/// Instances of `Fd` can be constructed with [`stdout()`] and [`stderr()`].
pub struct Fd(i32);

impl core::fmt::Write for Fd {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        if sys::write(self.0, s.as_bytes()) != -1 {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

/// Construct raw file descriptor for `stdout`.
pub fn stdout() -> Fd {
    Fd(1)
}

/// Construct raw file descriptor for `stderr`.
pub fn stderr() -> Fd {
    Fd(2)
}

/// Write format string to `stdout`, swallowing any errors.
#[macro_export]
macro_rules! println {
    () => ({
        $crate::println!("")
    });
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = writeln!(&mut $crate::stdout(), $($arg)*);
    });
}

/// Write format string to `stderr`, swallowing any errors.
#[macro_export]
macro_rules! eprintln {
    () => ({
        $crate::eprintln!("")
    });
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = writeln!(&mut $crate::stderr(), $($arg)*);
    });
}

/// Wrapper to invoke `riscv` linux syscalls.
///
/// Syscalls are generated according to the linux [syscall(2)][man-syscall] abi.
///
/// # Invocation
/// ```
/// Arch/ABI  Instruction  System  Ret  Ret
///                        call #  val  val2
/// ----------------------------------------
/// riscv     ecall        a7      a0   a1
/// ```
///
/// # Arguments
/// ```
/// Arch/ABI  arg1  arg2  arg3  arg4  arg5  arg6
/// --------------------------------------------
/// riscv     a0    a1    a2    a3    a4    a5
/// ```
///
/// [man-syscall]: https://man7.org/linux/man-pages/man2/syscall.2.html
#[cfg(target_arch = "riscv64")]
pub mod sys {
    use core::arch::asm;

    // Include generated syscall numbers (build.rs).
    include!(concat!(env!("OUT_DIR"), "/syscalls.rs"));

    #[inline]
    pub fn write(fd: i32, buf: &[u8]) -> i32 {
        let mut ret;
        unsafe {
            asm!(
                "ecall",
                in("a7") SYS_WRITE,
                inlateout("a0") fd => ret,
                in("a1") buf.as_ptr(),
                in("a2") buf.len(),
            );
        }
        ret
    }

    #[inline]
    pub fn exit(status: i32) -> ! {
        unsafe {
            asm!(
                "ecall",
                in("a7") SYS_EXIT,
                in("a0") status,
                options(noreturn),
            );
        }
    }
}
