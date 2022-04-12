#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::{
        println,
        tests::qemu_exit::{exit_qemu, QemuExitCode},
    };

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}
