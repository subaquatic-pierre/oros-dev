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

mod init;
mod interrupts;
mod port;
mod test_utils;
mod vga;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_utils::panic_handler(info);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init::init();
    println!("The NEWEST OS there is {}", "!");

    // x86_64::instructions::interrupts::int3();

    // trigger triple fault
    println!("It did not crash");

    // unsafe {
    //     *(0xdeadbeef as *mut &str) = "Triple fault";
    // }

    #[cfg(test)]
    test_main();

    loop {}
}
