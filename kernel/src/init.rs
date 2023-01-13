use bootloader_api::{entry_point, BootInfo};
use x86_64::instructions;
use x86_64::VirtAddr;

use crate::{
    interrupts,
    memory::{self, allocator, frame},
};

pub fn init(boot_info: &'static mut BootInfo) {
    // initialize interrupts and GDT
    interrupts::idt::init_idt();
    interrupts::gdt::init();
    unsafe { interrupts::pic::PICS.lock().initialize() };

    // enable interrupts
    instructions::interrupts::enable();

    // let physical_offset = match boot_info.physical_memory_offset.into_option() {
    //     Some(num) => num,
    //     None => 0,
    // };

    let phys_mem_offset_addr =
        VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap_or(0));

    // initialize mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset_addr) };
    // allocator
    let mut frame_allocator =
        unsafe { frame::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    // heap allocatotion init
    memory::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // initialize memory
}
