#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oros::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use oros::allocator;
use oros::hlt_loop;
use oros::init;
use oros::memory;

entry_point!(test_kernel_main);

fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    // get the physical memory offset

    // initialize RAM
    init::init();

    let phys_mem_offset = boot_info.physical_memory_offset;
    let phys_mem_offset_addr = VirtAddr::new(phys_mem_offset);

    // initialize mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset_addr) };
    // allocator
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // heap allocatotion init
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

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
    use oros::allocator::HEAP_SIZE;

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
    fn simple_allocation() {
        let heap_val_1 = Box::new(41);
        let heap_val_2 = Box::new(7);

        assert_eq!(*heap_val_1, 41);
        assert_eq!(*heap_val_2, 7);
    }
}
