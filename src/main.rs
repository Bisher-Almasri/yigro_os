#![no_std]
#![no_main]

use core::panic::PanicInfo;


static MSG: &[u8] = b"Test!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in MSG.iter().enumerate() {
         unsafe {
            *vga_buf.offset(i as isize * 2) = byte; // text
            *vga_buf.offset(i as isize * 2 + 1) = 0xb; // color
        }
    }
    loop {}
}

// since we are using nostd we need ot make our wn panic handler
// this is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
