#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oros::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use oros::hlt_loop;
use oros::init;

entry_point!(test_kernel_main);

fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    // get the physical memory offset

    // initialize RAM
    init::init(boot_info);

    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oros::test_utils::panic_handler(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec::Vec};
    use oros::memory::allocator::HEAP_SIZE;

    #[test_case]
    fn large_vec() {
        let n = 1000;
        let mut vec = Vec::new();

        for i in 0..n {
            vec.push(i)
        }

        assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    }

    #[test_case]
    fn many_boxes() {
        for i in 0..HEAP_SIZE {
            let x = Box::new(i);
            assert_eq!(*x, i)
        }
    }

    #[test_case]
    fn many_boxes_long_lived() {
        let long_lived = Box::new(1);

        for i in 0..HEAP_SIZE {
            let x = Box::new(i + 1);
            assert_eq!(i + 1, *x);
        }

        assert_eq!(1, *long_lived);
    }

    #[test_case]
    fn simple_allocation() {
        let heap_val_1 = Box::new(41);
        let heap_val_2 = Box::new(7);

        assert_eq!(*heap_val_1, 41);
        assert_eq!(*heap_val_2, 7);
    }
}
