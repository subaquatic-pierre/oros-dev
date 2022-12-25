//! Interrupts provide a way to notify the CPU from attached hardware devices.

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// The Intel 8259 is a programmable interrupt controller (PIC) introduced in 1976. It has long been replaced by the newer APIC, but its interface is still supported on current systems for backwards compatibility reasons.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
