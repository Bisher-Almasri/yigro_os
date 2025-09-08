#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(yigro_os::test_runner)]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use yigro_os::{
    allocator, println,
    task::{executor::Executor, keyboard, Task},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use yigro_os::memory::{self, BootInfoFrameAllocator};
    println!("skib{}", "idi");
    use x86_64::{structures::paging::Page, VirtAddr};

    yigro_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    let x = Box::new(41);

    println!("it didntr crqsh");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

// since we are using nostd we need ot make our wn panic handler
// this is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    yigro_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yigro_os::test_panic_handler(info);
}
