use crate::{
    ansi_colors::Yellow,
    qemu_exit::{exit_qemu, QemuExitCode},
    serial_println,
    testable::Testable,
};

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
