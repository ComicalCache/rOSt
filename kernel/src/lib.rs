#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(
    abi_x86_interrupt,
    generic_const_exprs,
    core_intrinsics,
    asm_const,
    naked_functions
)]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use test_framework::{
    ansi_colors,
    qemu_exit::{exit_qemu, QemuExitCode},
    serial_println,
};

mod init;
pub use init::{hlt_loop, init, register_driver, register_syscall, reload_drivers};

use crate::logger::Logger;

mod interrupts;
mod user_mode;
pub use user_mode::run_in_user_mode;
pub mod logger;
mod memory;
pub mod structures;

#[cfg(debug_assertions)]
mod debug;

lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Option<Box<dyn Logger>>>> = Arc::from(Mutex::new(None));
}

/// Function called when a panic occurs
pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[PANIC]"));
    serial_println!("Error: {}\n", info);
    init::hlt_loop();
}

/// Function called when a panic while testing occurs
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[PANIC]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    init::hlt_loop();
}
