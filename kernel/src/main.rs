//! Main entry point for oros_kernel kernel
//! operating system kernel developed in Rust

// TODO: Remove warnings from code
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// unstable features
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
// test attributes
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern alloc crate to be compiled with binary
extern crate alloc;

use core::panic::PanicInfo;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Page, PageTable, Size4KiB, Translate};
use x86_64::VirtAddr;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};

use oros_kernel::memory::{self, allocator, frame};
use oros_kernel::task::{executor::Executor, keyboard, Task};
use oros_kernel::{hlt_loop, init, println, test_utils};

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_utils::panic_handler(info);
}

/// This function is called on system panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hlt_loop();
}

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

// Main entry point function
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // get the physical memory offset

    // initialize RAM
    init::init(boot_info);

    // print the os is working
    println!("The NEWEST OS there is {}", "!");

    // TODO:
    // Move allocator init logic into
    // main init method

    // create page to test VGA buffer
    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0xb8000));
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

    println!("It did not crash!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_key_presses()));
    executor.run();

    // run tests if 'cargo test'
    #[cfg(test)]
    test_main();

    // start system loop, idle CPU if no current instructions
    // hlt_loop()
}

async fn example_number() -> u32 {
    42
}

async fn example_task() {
    let number = example_number().await;
    println!("{number}");
}
