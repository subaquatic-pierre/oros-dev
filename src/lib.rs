//! Main kernel library entry for oros
//! linux type OS kernel

// TODO: Remove allow dead code
// most below used for dev purpose
// must refactor code to remove warnings
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![no_std] // don't link the Rust standard library

// unstable features
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
// test attributes
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

// import kernel modules
pub mod init;
pub mod interrupts;
pub mod memory;
pub mod port;
pub mod test_utils;
pub mod vga;

/// main entry point used when cargo test
#[cfg(test)]
entry_point!(test_kernel_main);

/// Only run lib test kernel on cargo test
/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    // initialize kernel
    init::init(boot_info);

    // start test runner
    test_main();

    // system halt loop used to preserve CPU cycle
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_utils::panic_handler(info);
}

/// x86 system halt loop
/// used to preserve CPU cycle
pub fn hlt_loop() -> ! {
    use x86_64::instructions;
    loop {
        instructions::hlt();
    }
}

// define memory allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {layout:?}");
}
