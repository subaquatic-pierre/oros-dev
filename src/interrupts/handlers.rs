use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::handlers;
use crate::println;

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