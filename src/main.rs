#![no_std]
#![no_main]

use user_sw::{eprintln, println, sys};

fn main() {
    println!("Hello {} from rust main().", 1337);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    sys::exit(0);
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    sys::exit(42);
}

// Since we disable the atomic isa extension, the compiler emits calls to software emulation. We
// provide the stub to make the linker happy for now.
#[no_mangle]
pub fn __atomic_load_8() {
    panic!("__atomic_load_8 not implemented!");
}
