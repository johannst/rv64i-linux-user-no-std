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
