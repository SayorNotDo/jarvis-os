// 禁用与标准库的链接
#![no_std]
// 禁用预定义的入口点
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvis::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use jarvis::{hlt_loop, init, println};

// macro define the entry point like extern "C" fn _start()
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    jarvis::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // // invoke pagefault exception
    // let ptr = 0x2031b2 as *mut u8;
    // unsafe {
    //     let x = *ptr;
    // }
    // println!("read worked");

    // unsafe {
    //     *ptr = 42;
    // };

    // println!("write work");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    jarvis::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    jarvis::hlt_loop();
}

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
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
