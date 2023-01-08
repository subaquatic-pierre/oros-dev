use lazy_static::lazy_static;
use pc_keyboard::{layouts, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{
    instructions::port::{PortGeneric, ReadWriteAccess},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use super::{
    handlers,
    pic::{InterruptIndex, PICS},
};

use crate::hlt_loop;
use crate::{port::num::PortNumber, print, println};

/// Breakpoint interrupt handler
pub extern "x86-interrupt" fn breakpiont_handler(stack_frame: InterruptStackFrame) {
    println!("BREAKPOINT EXCEPTION:");
    println!("{stack_frame:#?}");
}

/// Double fault interrupt handler
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}")
}

/// Timer interrupt handler
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}

/// Keyboard interrupt handler
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(PortNumber::Keyboard.into());
    let scancode: u8 = unsafe { port.read() };

    // use executor to add scancode to task queue
    // used for concurrency
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into());
    }
}

/// memory paging interupt handler
pub extern "x86-interrupt" fn paging_fault_handler(
    stack_frame: InterruptStackFrame,
    err_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {err_code:?}");
    println!("{stack_frame:#?}");
    hlt_loop();
}
