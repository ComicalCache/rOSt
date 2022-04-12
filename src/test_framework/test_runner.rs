#[cfg(test)]
use super::testable::Testable;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::{
        serial_println,
        test_framework::qemu_exit::{exit_qemu, QemuExitCode},
    };

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}
