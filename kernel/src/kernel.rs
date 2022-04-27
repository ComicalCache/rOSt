#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

extern crate alloc;

use core::panic::PanicInfo;

pub use crate::interrupts::gdt;

use test_framework::{
    ansi_colors,
    qemu_exit::{exit_qemu, QemuExitCode},
    serial_println,
};

mod init;
pub use init::{hlt_loop, init, register_driver, reload_drivers};
mod interrupts;
pub mod logger;
mod memory;
pub mod structures;

#[cfg(debug_assertions)]
mod debug;

/// Function called when a panic while testing occurs
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[failed]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    init::hlt_loop();
}
