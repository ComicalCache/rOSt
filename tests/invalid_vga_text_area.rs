#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_testing::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os_testing::{
    ansi_colors::{Green, Yellow, Red},
    serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode}, interface::VGA_TEXT_BUFFER_INTERFACE,
};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    serial_println!("{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);

    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    let test_count = tests.len();

    if test_count > 0 {
        serial_println!(
            "{} {} {}",
            Yellow("Running"),
            test_count,
            Yellow("test(s):")
        );

        for test in tests {
            test();
            serial_println!("{}", Red("[test did not panic]"));
            exit_qemu(QemuExitCode::Failed);
        }
    }

    exit_qemu(QemuExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// ! insert your tests bellow

#[test_case]
fn out_of_bounds() {
    VGA_TEXT_BUFFER_INTERFACE.lock().set_pos(1000, 1000);
}
