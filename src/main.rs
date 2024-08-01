// 禁用与标准库的链接
#![no_std]

// 禁用预定义的入口点
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // 防止函数名重整
pub extern "C" fn _start() -> ! {
    loop {

    }
}
