#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(jarvis::test_runner)]

use core::panic::PanicInfo;
use jarvis::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jarvis::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
