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

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::registers::control::Cr3;

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

/// Main entry point function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init::init();
    println!("The NEWEST OS there is {}", "!");

    // x86_64::instructions::interrupts::int3();

    // trigger triple fault
    println!("It did not crash");

    // trigger page fault
    let ptr: *mut u32 = 0x2074ee as *mut u32;
    println!("Able to read from addr 0x2074ee");
    // unsafe {
    //     *ptr = 42;
    // }
    // println!("Unable to write to that address");

    // unsafe {
    //     *(0xdeadbeef as *mut &str) = "Triple fault";
    // }

    // read CPU page table regsiters
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table);
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    hlt_loop()
}
