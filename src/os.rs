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
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::structures::kernel_information::KernelInformation;
use utils::format_size;

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

pub fn kernel_main(_: KernelInformation) {
    //let test = Box::new(4);
    //log_println!("New boxed value: {:#?}", test);
    //log_println!("im not dying :)");
    log_println!("Getting all disks...");
    let disks = ata::get_all_disks();
    log_println!("Got {} disks, taking the non-bootable one...", disks.len());
    let mut disk = disks
        .into_iter()
        .map(|mut disk| (disk.has_bootloader(), disk))
        .find(|(boot, _)| !boot.unwrap_or(true))
        .expect("No non-bootable disk found")
        .1;
    log_println!("Got a disk, looking for partitions...");
    let mut partitions = disk.get_partitions().expect("Error getting partitions");
    if partitions.len() == 0 {
        log_println!("No partitions found, creating a new one...");
        let partition_size = disk.descriptor.lba_48_addressable_sectors as u32 / 2;
        disk.create_partition(partition_size, 0xED)
            .expect("Error creating partition");
        log_println!("Partition created, double-checking...");
        partitions = disk.get_partitions().expect("Error getting partitions");
        if partitions.len() == 0 {
            log_println!("No partitions found, giving up.");
            return;
        }
    }
    log_println!("Found {} partitions:", partitions.len());
    for partition in partitions {
        log_println!(
            "{:8} - starting at {:8X}",
            format_size(partition.descriptor.sectors * 512),
            partition.descriptor.start_lba
        )
    }
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
pub fn kernel_test(_kernel_info: KernelInformation) {
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
