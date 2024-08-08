// 禁用与标准库的链接
#![no_std]
// 禁用预定义的入口点
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvis::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jarvis::println;

#[no_mangle] // 防止函数名重整
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    jarvis::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jarvis::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
