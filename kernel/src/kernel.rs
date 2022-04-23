#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

extern crate alloc;

use alloc::vec::Vec;
use core::panic::PanicInfo;

use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::VirtAddr;

use structures::driver::Registrator;
use structures::kernel_information::KernelInformation;

pub use crate::interrupts::gdt;
use crate::memory::page_table::{BootInfoFrameAllocator, MEMORY_MAPPER};
use crate::structures::driver::Driver;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;

pub mod interrupts;
pub mod logger;
pub mod memory;
pub mod structures;
pub mod test_framework;

lazy_static! {
    static ref REGISTERED_DRIVERS: Mutex<Vec<Registrator>> = Mutex::new(Vec::new());
    static ref INITIALIZED_DRIVERS: Mutex<Vec<Driver>> = Mutex::new(Vec::new());
}

/// Initialises the components of the OS, **must** be called before any other functions.
pub fn init(boot_info: &'static BootInfo) -> KernelInformation {
    let kernel_info = KernelInformation::new(boot_info);
    interrupts::init_gdt();
    interrupts::init_idt();
    unsafe {
        // can cause undefined behaviour if the offsets were not set correctly
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();

    unsafe {
        memory::page_table::init(VirtAddr::new(
            boot_info
                .physical_memory_offset
                .into_option()
                .expect("physical memory mapping not set"),
        ))
    };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    let mut mapper = MEMORY_MAPPER.lock();
    memory::heap::init_heap(mapper.as_mut().unwrap(), &mut frame_allocator)
        .expect("heap initialization failed");

    kernel_info
}

pub fn reload_drivers(kernel_info: KernelInformation) {
    let mut initialized_drivers = INITIALIZED_DRIVERS.lock();
    initialized_drivers.clear();
    initialized_drivers.extend(
        REGISTERED_DRIVERS
            .lock()
            .iter()
            .map(|registrator| registrator(kernel_info)),
    );
}

pub fn register_driver(registrator: Registrator) {
    REGISTERED_DRIVERS.lock().push(registrator);
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
