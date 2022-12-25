use crate::interrupts;

pub fn init() {
    interrupts::idt::init_idt();
    interrupts::gdt::init();
}
