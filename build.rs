// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022, Johannes Stoelp <dev@memzero.de>

/// Generate rust constants for linux riscv syscall numbers.
///
/// Syscall numbers are parsed from the `linux/unistd.h` header.
/// This script uses the `riscv64-linux-gnu-*` tools to find to header, make sure those tools are
/// in the PATH.
///
/// For simplicity, this script unwraps any Option/Result because we want to fail in case of any
/// error.

fn main() {
    // Get sysroot to find header location.
    let out = std::process::Command::new("riscv64-linux-gnu-gcc")
        .arg("-print-sysroot")
        .output()
        .unwrap();
    let sysroot = std::str::from_utf8(&out.stdout).unwrap();
    let header = format!("{}/include/linux/unistd.h", sysroot.trim_end());

    // The header should "basically" never change.
    println!("cargo:rerun-if-changed={}", header);

    // Run header through preprocessor and dump macro definitions.
    let out = std::process::Command::new("riscv64-linux-gnu-cpp")
        .arg("-dM")
        .arg(header)
        .output()
        .unwrap();
    let defines = std::str::from_utf8(&out.stdout).unwrap();

    let mut output = String::with_capacity(256);

    // Parse out lines of the format
    //   #define __NR_<syscall> <nr>
    // and generate output strings.
    for line in defines.lines() {
        let line = match line.strip_prefix("#define __NR_") {
            Some(line) => line,
            None => continue,
        };

        let (sys, nr) = match line.split_once(' ') {
            Some(split) => split,
            None => continue,
        };

        let nr = match nr.parse::<usize>() {
            Ok(nr) => nr,
            Err(_) => continue,
        };

        output.push_str(&format!(
            "#[allow(unused)] const SYS_{}: usize = {};\n",
            sys.to_uppercase(),
            nr
        ));
    }

    // Write out rust file with syscall numbers.
    let outfile = format!("{}/syscalls.rs", std::env::var("OUT_DIR").unwrap());
    std::fs::write(outfile, output).unwrap();
}
