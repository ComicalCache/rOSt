#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

extern crate alloc;

use core::panic::PanicInfo;

pub use crate::interrupts::gdt;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;

mod init;
pub use init::{hlt_loop, init, register_driver, reload_drivers};
mod interrupts;
pub mod logger;
mod memory;
pub mod structures;
mod test_framework;
pub use test_framework::{serial, test_runner};

#[cfg(debug_assertions)]
mod debug;

/// Function called when a panic while testing occurs
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[failed]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    init::hlt_loop();
}
