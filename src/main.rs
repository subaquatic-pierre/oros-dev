#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::fmt::Write;
use core::panic::PanicInfo;

mod vga;

use vga::writer::{WRITER as writer};

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

static HELLO_WORLD: &[u8] = b"Hello, World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("The NEWEST OS there is {}", "?!!$#!");
    loop {}
}
