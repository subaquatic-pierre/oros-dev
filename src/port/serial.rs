use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

use super::num::PortNumber;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(PortNumber::QemuDebugExit.into());
        port.write(exit_code as u32);
    }
}

#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    })
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::port::serial::_serial_print(format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! serial_println {
    ()=>($crate::serial_print!("\n"));
    ($fmt:expr) => {
        ($crate::serial_print!(concat!($fmt,"\n")))
    };
    ($fmt:expr,$($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
