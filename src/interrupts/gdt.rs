//! The Global Descriptor Table (GDT) is a relic that was used for memory segmentation before paging became the de facto standard. However, it is still needed in 64-bit mode for various things, such as kernel/user mode configuration or TSS loading.

use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::{
    instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    },
    structures::gdt::SegmentSelector,
};
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// selector struct used to set CS (Code Segment) segment and TSS (Task State Segment) segment on  CPU
struct Selectors {
    cs_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    /// The GDT is a structure that contains the segments of the program. It was used on older architectures to isolate programs from each other before paging became the standard.
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors{cs_selector,tss_selector})
    };
}

lazy_static! {
    /// The Interrupt Stack Table (IST) is part of an old legacy structure called Task State Segment (TSS). The TSS used to hold various pieces of information (e.g., processor register state) about a task in 32-bit mode and was, for example, used for hardware context switching. However, hardware context switching is no longer supported in 64-bit mode and the format of the TSS has changed completely.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // create new stack table for interupt stack
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // return the address at the end of the stack
            // stacks grow from higher address to lower address
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        tss
    };
}

/// The Interrupt Stack Table (IST)
/// Loads TSS (Task State Segment) and
/// CS (Code Segment regsiter) using the
/// GDT (Global Descriptor Table)
pub fn init() {
    GDT.0.load();

    unsafe {
        load_tss(GDT.1.tss_selector);
        CS::set_reg(GDT.1.cs_selector);
    }
}
