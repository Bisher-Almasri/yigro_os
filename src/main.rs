#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(yigro_os::test_runner)]

use core::panic::PanicInfo;
use yigro_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("skib{}", "idi");

    yigro_os::init();

    #[cfg(test)]
    test_main();

    yigro_os::hlt_loop();
}

// since we are using nostd we need ot make our wn panic handler
// this is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    yigro_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yigro_os::test_panic_handler(info);
}

