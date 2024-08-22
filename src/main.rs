// 禁用与标准库的链接
#![no_std]
// 禁用预定义的入口点
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvis::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::{
    panic::PanicInfo,
    sync::atomic::{AtomicU32, Ordering},
};
use jarvis::println;

// macro define the entry point like extern "C" fn _start()
entry_point!(kernel_main);

// 定义静态变量，表示可用于调用工作的CPU数量
static CPU_COUNT: AtomicU32 = AtomicU32::new(0);

// 当前活跃CPU的数量
#[inline(always)]
fn cpu_count() -> u32 {
    CPU_COUNT.load(Ordering::Relaxed)
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    CPU_COUNT.store(cpu_count(), Ordering::SeqCst);
    // use jarvis::allocator;
    // use jarvis::memory;

    // 导入Translate特性以使用提供的translate_addr方法而非自定义
    use x86_64::VirtAddr;
    println!("Core: {}", cpu_count());

    println!("Hello World{}", "!");

    jarvis::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // initialize mapper
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // 初始化页帧分配器
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

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
    // let mut executor = SimpleExecutor::new();
    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.run();
    /************** Test Code End **************/

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    jarvis::hlt_loop();
}

/************** Test Code Start **************/
// async fn async_number() -> u32 {
//     42
// }

// async fn example_task() {
//     let number = async_number().await;
//     println!("async number: {}", number);
// }
/************** Test Code End **************/

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
