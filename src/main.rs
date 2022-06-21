// remove standard libs
#![no_std]

// disable rust runtime libary-we do not have access (no_std)
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// overwrite entrypoint inplace of absent rust runtime (no main)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default (when using Linux conventions)
    loop{}
}
