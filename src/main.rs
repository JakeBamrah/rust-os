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
static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default (when using Linux conventions)
    let vga_buffer = 0xb8000 as *mut u8;
    // iterate over the bytes of the static string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // use offset to write string byte and color byte (0xb)
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}
