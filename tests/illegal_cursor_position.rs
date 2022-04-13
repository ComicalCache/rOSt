#![no_std]
#![no_main]

use core::panic::PanicInfo;

use os_core::{
    ansi_colors::{Green, Red, Yellow},
    serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode},
    vga::text::interface::VGA_TEXT_BUFFER_INTERFACE,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{} 1 {}", Yellow("Running"), Yellow("test(s):"));

    serial_println!(
        "illegal_cursor_position::illegal_cursor_position_test...\t{}",
        Green("[ok]")
    );
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    illegal_cursor_position_test();
    serial_println!(
        "illegal_cursor_position::illegal_cursor_position_test...\t{}",
        Red("[failed]")
    );
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn illegal_cursor_position_test() {
    VGA_TEXT_BUFFER_INTERFACE.lock().set_pos(1000, 1000);
}
