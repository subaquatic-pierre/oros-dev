use core::panic::PanicInfo;

use crate::{println, serial_print, serial_println};

/// Testable trait used for all test cases
/// implements printing functionality for any test case
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}

// custom test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::port::serial::{exit_qemu, QemuExitCode};

    serial_println!("Running {} tests ...", tests.len());
    for test in tests {
        test.run()
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    use crate::port::serial::{exit_qemu, QemuExitCode};

    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    println!("{info}");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}
