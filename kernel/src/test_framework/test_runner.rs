use crate::{
    serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode},
};

use super::testable::Testable;
use crate::test_framework::ansi_colors::Yellow;

/// Rusts test runner function that is called to run all annotated tests.
#[allow(dead_code)]
pub fn test_runner(tests: &[&dyn Testable]) {
    let test_count = tests.len();
    if test_count > 0 {
        serial_println!(
            "{} {} {}",
            Yellow("Running"),
            test_count,
            Yellow("test(s):")
        );
        for test in tests {
            test.run();
        }
    }

    exit_qemu(QemuExitCode::Success);
}
