#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

// ########################################################
// #   This library is the core of the operating system   #
// ########################################################

pub use crate::interrupts::gtd;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;
pub use crate::vga::text::interface;

use core::panic::PanicInfo;

pub mod interrupts;
pub mod low_level;
pub mod test_framework;
pub mod vga;

pub fn init() {
    interrupts::init_gdt();
    interrupts::init_idt();
}

/// `cargo test` entry point
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// ###########################################################
// #   Only defines a panic handler for the test framework   #
// ###########################################################

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", ansi_colors::Red("[failed]\n"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
