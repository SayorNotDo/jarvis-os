// 禁用与标准库的链接
#![no_std]
// 禁用预定义的入口点
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvis::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::{iter::Take, panic::PanicInfo};
use jarvis::{
    memory::BootInfoFrameAllocator,
    println,
    task::{simple_executor::SimpleExecutor, Task},
};

// macro define the entry point like extern "C" fn _start()
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use jarvis::allocator;
    use jarvis::memory;

    // 导入Translate特性以使用提供的translate_addr方法而非自定义
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    jarvis::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // initialize mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    /************** Test Code End **************/
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // 通过新的映射将字符串`New!`写到屏幕上
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let heap_value = Box::new(41);

    // println!("heap_value at {:p}", heap_value);
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );
    // core::mem::drop(reference_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );
    //
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
    /************** Test Code End **************/

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    jarvis::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    jarvis::hlt_loop();
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
