//! Main entry point for oros kernel
//! operating system kernel developed in Rust

// TODO: Remove warnings from code
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// test attributes
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern alloc crate to be compiled with binary
extern crate alloc;

use core::panic::PanicInfo;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Page, PageTable, Size4KiB, Translate};
use x86_64::VirtAddr;

use oros::memory::{self, allocator, frame};
use oros::{hlt_loop, init, println, test_utils};

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

entry_point!(kernel_main);

// Main entry point function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
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

    // test allocator
    // perfom main logic
    let x = Box::new(41);
    let x_ptr: *const u64 = &*x;
    println!("The coolest box value: {x} at address {:?}", x_ptr);

    // create vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i)
    }
    println!("The vector address is {:p}", vec.as_slice());

    // create referrence counted vector -> will be freed when count reaches 0
    let ref_count = Rc::new(vec![1, 2, 3]);
    let cloned_ref = ref_count.clone();
    println!(
        "current referrence counr is {}",
        Rc::strong_count(&cloned_ref)
    );
    core::mem::drop(ref_count);
    println!("reference count is {} now ", Rc::strong_count(&cloned_ref));

    println!("It did not crash!");

    // run tests if 'cargo test'
    #[cfg(test)]
    test_main();

    // start system loop, idle CPU if no current instructions
    hlt_loop()
}
