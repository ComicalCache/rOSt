#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::{
        serial_println,
        test_framework::qemu_exit::{exit_qemu, QemuExitCode},
    };

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}
