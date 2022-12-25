// TODO: Remove allow dead code
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![no_std] // don't link the Rust standard library

// unstable features
#![feature(abi_x86_interrupt)]
// test attributes
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod init;
pub mod interrupts;
pub mod port;
pub mod test_utils;
pub mod vga;

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init::init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_utils::panic_handler(info)
}
