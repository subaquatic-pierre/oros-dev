use x86_64::instructions::interrupts::enable;

use crate::interrupts;

pub fn init() {
    // initialize interrupts and GDT
    interrupts::idt::init_idt();
    interrupts::gdt::init();
    unsafe { interrupts::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // initialize memory
}
