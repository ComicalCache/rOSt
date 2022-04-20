#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

extern crate alloc;

use core::panic::PanicInfo;

use x86_64::VirtAddr;

use crate::memory::page_table::BootInfoFrameAllocator;
use crate::structures::driver::Driver;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use structures::driver::Registrator;
use structures::kernel_information::KernelInformation;

pub mod memory;
pub use crate::interrupts::gdt;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;

pub mod interrupts;
pub mod logger;
pub mod structures;
pub mod test_framework;

lazy_static! {
    static ref REGISTERED_DRIVERS: Mutex<[Option<Registrator>; 256]> = Mutex::new([None; 256]);
    static ref INITIALIZED_DRIVERS: Mutex<[Option<Driver>; 256]> = Mutex::new([None; 256]);
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

    let mut mapper = unsafe {
        memory::page_table::init(VirtAddr::new(
            boot_info
                .physical_memory_offset
                .into_option()
                .expect("physical memory mapping not set"),
        ))
    };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    memory::heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    kernel_info
}

pub fn reload_drivers(kernel_info: KernelInformation) {
    let mut initialized_drivers = INITIALIZED_DRIVERS.lock();
    for (index, driver_generator) in REGISTERED_DRIVERS.lock().iter().enumerate() {
        if let Some(driver) = driver_generator {
            initialized_drivers[index] = Some(driver(kernel_info));
        } else {
            break;
        }
    }
}

pub fn register_driver(registrator: Registrator) {
    for driver in REGISTERED_DRIVERS.lock().iter_mut() {
        if driver.is_none() {
            driver.replace(registrator);
            return;
        }
    }
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
