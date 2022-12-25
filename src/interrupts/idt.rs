use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::{gdt, handlers, pic::InterruptIndex};
use crate::println;

lazy_static! {
    /// Interrupt descriptor table
    /// used to create index of interrupt codes and register handlers
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpiont_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(handlers::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.into()].set_handler_fn(handlers::timer_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

#[cfg(test)]
mod test {
    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
