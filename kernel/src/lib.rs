//! Main kernel library entry for oros
//! linux type OS kernel

// TODO: Remove allow dead code
// most below used for dev purpose
// must refactor code to remove warnings
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
#![feature(trait_alias)]
#![no_std] // don't link the Rust standard library

// unstable features
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
// test attributes
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};

// import kernel modules
pub mod init;
pub mod interrupts;
pub mod memory;
pub mod port;
pub mod screen;
pub mod task;
pub mod test_utils;

// main entry point used when cargo test
#[cfg(test)]
entry_point!(test_kernel_main, &BOOTLOADER_CONFIG);

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

// Only run lib test kernel on cargo test
// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
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
