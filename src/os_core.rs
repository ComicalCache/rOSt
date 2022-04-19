#![no_std]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

// ########################################################
// #   This library is the core of the operating system   #
// ########################################################

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::boot_info::{FrameBuffer, MemoryRegions};
use x86_64::VirtAddr;

pub use crate::interrupts::gdt;
use crate::memory::page_table::BootInfoFrameAllocator;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;

pub mod basic_drivers;
pub mod interrupts;
pub mod logger;
pub mod memory;
pub mod structures;
pub mod test_framework;

/// Initialises the components of the OS, **must** be called before any other functions.
pub fn init(framebuffer: &'static mut FrameBuffer, physical_memory_offset: u64, memory_regions: &'static MemoryRegions) {
    logger::init(framebuffer);
    interrupts::init_gdt();
    interrupts::init_idt();
    unsafe {
        // can cause undefined behaviour if the offsets were not set correctly
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();

    let mut mapper = unsafe {
        memory::page_table::init(VirtAddr::new(physical_memory_offset))
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(memory_regions)
    };

    memory::heap::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}

/// Endless loop calling halt continuously.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Function called when a panic while testing occurs
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", ansi_colors::Red("[failed]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
}
