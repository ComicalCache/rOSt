#![no_std]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

// ########################################################
// #   This library is the core of the operating system   #
// ########################################################

use bootloader::boot_info::FrameBuffer;

pub use crate::interrupts::gdt;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;

use core::panic::PanicInfo;

pub mod basic_drivers;
pub mod interrupts;
pub mod logger;
pub mod memory;
pub mod structures;
pub mod test_framework;

/// Initialises the components of the OS, **must** be called before any other functions.
pub fn init(framebuffer: &'static mut FrameBuffer) {
    logger::init(framebuffer);
    interrupts::init_gdt();
    interrupts::init_idt();
    unsafe {
        // can cause undefined behaviour if the offsets were not set correctly
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

/// Endless loop calling halt continuously.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Function called when a panic while testing occures
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[failed]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
}
