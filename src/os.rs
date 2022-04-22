#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    generic_const_exprs,
    core_intrinsics,
    alloc_error_handler
)]
#![test_runner(kernel::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ata::{PRIMARY_ATA_BUS, SECONDARY_ATA_BUS};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::structures::kernel_information::KernelInformation;
use vga::{vga_buffer::VGADeviceFactory, vga_color, vga_core::Clearable};
extern crate alloc;

use alloc::{format, string::String};
use core::alloc::Layout;

use kernel::log_println;

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    let kernel_info = kernel::init(boot_info);
    kernel::register_driver(vga::driver_init);
    kernel::register_driver(ata::driver_init);
    kernel::reload_drivers(kernel_info);

    #[cfg(test)]
    kernel_test(kernel_info);
    #[cfg(not(test))]
    kernel_main(kernel_info);

    kernel::hlt_loop();
}

pub fn kernel_main(kernel_info: KernelInformation) {
    let mut device = VGADeviceFactory::from_kernel_info(kernel_info);
    device.clear(vga_color::BLACK);

    let disk_tests = [
        (PRIMARY_ATA_BUS, true),
        (PRIMARY_ATA_BUS, false),
        (SECONDARY_ATA_BUS, true),
        (SECONDARY_ATA_BUS, false),
    ];
    for (index, (bus, master)) in disk_tests.iter().enumerate() {
        let mut bus_instance = bus.lock();
        let descriptor = bus_instance.identify(*master);
        if let Ok(descriptor) = descriptor {
            log_println!(
                "Found a disk: {} ({})",
                descriptor.model_number().trim(),
                format_size(descriptor.lba_48_addressable_sectors * 512)
            );
            let partitions = bus_instance.get_partitions(*master);
            log_println!("  Partitions: {:?}", partitions.ok().unwrap());
        }
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{}B", bytes);
    }
    if bytes < 1024 * 1024 {
        return format!("{}KB", bytes / 1024);
    }
    if bytes < 1024 * 1024 * 1024 {
        return format!("{}MB", bytes / 1024 / 1024);
    }
    format!("{}GB", bytes / 1024 / 1024 / 1024)
}

/// Panic handler for the OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::log_print!("{}", info);
    kernel::hlt_loop();
}

/// This is the main function for tests.
#[cfg(test)]
pub fn kernel_test(kernel_info: KernelInformation) {
    test_main();
}

/// Panic handler for the OS in test mode.
#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
