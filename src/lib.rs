#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, arbitrary_enum_discriminant)]
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

// ########################################################
// # This library is used as base for testing  and can    #
// # mostly be ignored while developing the kernel.       #
// # Added features must be included here to be available #
// # for testing.                                         #
// ########################################################

use core::panic::PanicInfo;

use crate::test_framework::qemu_exit::{exit_qemu, QemuExitCode};

pub use crate::test_framework::serial; // makes serial_print! and serial_println! available
pub use crate::test_framework::ansi_colors; // makes colors available
pub use crate::vga::text::interface; // makes the VgaTextBufferInterface available

mod low_level;
pub mod test_framework;
pub mod vga;


pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", ansi_colors::Red("[failed]\n"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// `cargo test` entry point
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
