use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::{
    handlers,
    pic::{InterruptIndex, PICS},
};
use crate::{print, println};

pub extern "x86-interrupt" fn breakpiont_handler(stack_frame: InterruptStackFrame) {
    println!("BREAKPOINT EXCEPTION:");
    println!("{stack_frame:#?}");
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}")
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}
