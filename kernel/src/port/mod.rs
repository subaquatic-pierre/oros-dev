pub mod num;
pub mod serial;
use self::num::PortNumber;
use x86_64::instructions::port;
use x86_64::{
    instructions::port::{PortGeneric, PortWrite, ReadWriteAccess},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

pub struct Port;

impl Port {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<PortNumber>>(port: T) -> PortGeneric<PortNumber, ReadWriteAccess>
    where
        u16: From<T>,
    {
        port::Port::new(port.into())
    }
}
