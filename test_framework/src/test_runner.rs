use crate::{
    ansi_colors::Yellow,
    qemu_exit::{exit_qemu, QemuExitCode},
    testable::Testable,
};
use kernel::structures::kernel_information::KernelInformation;
use utils::serial_println;

pub static mut KERNEL_INFO: Option<KernelInformation> = None;

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
            test.run(unsafe { KERNEL_INFO }.clone().unwrap());
        }
    }

    exit_qemu(QemuExitCode::Success);
}
