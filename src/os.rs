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
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::alloc::Layout;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::VirtAddr;

use ata::constants::{PRIMARY_ATA_BUS, SECONDARY_ATA_BUS};
use os_core::log_println;
use os_core::memory::page_table;
use os_core::memory::page_table::{BootInfoFrameAllocator, MEMORY_MAPPER};
use os_core::{log_print, structures::kernel_information::KernelInformation};
use vga::{vga_buffer::VGADeviceFactory, vga_color, vga_core::Clearable};

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    let kernel_info = os_core::init(boot_info);
    os_core::register_driver(vga::driver_init);
    os_core::register_driver(ata::driver_init);
    os_core::reload_drivers(kernel_info);

    #[cfg(test)]
    kernel_test(kernel_info);
    #[cfg(not(test))]
    kernel_main(kernel_info);

    os_core::hlt_loop();
}

pub fn kernel_main(kernel_info: KernelInformation) {
    let mut device = VGADeviceFactory::from_kernel_info(kernel_info);
    device.clear(vga_color::BLACK);

    /*
    let disk_tests = [
        (PRIMARY_ATA_BUS, true),
        (PRIMARY_ATA_BUS, false),
        (SECONDARY_ATA_BUS, true),
        (SECONDARY_ATA_BUS, false),
    ];
    for (index, (bus, master)) in disk_tests.iter().enumerate() {
        let descriptor = bus.lock().identify(*master);
        match descriptor {
            Ok(descriptor) => {
                log_println!("{}:ATA device discovered", index);
                log_println!("  Is fixed: {:?}", descriptor.fixed_device);
                log_println!("  Is removable: {:?}", descriptor.removable_media);
                log_println!("  Is ATA device: {:?}", descriptor.is_ata_device);

                log_println!("  Cylinders: {}", descriptor.cylinders);
                log_println!("  Heads: {}", descriptor.heads);
                log_println!("  Sectors per Track: {}", descriptor.sectors_per_track);

                log_println!("  Vendor Unique: {:x?}", descriptor.vendor_unique);

                log_println!("  Serial Number: {:?}", descriptor.serial_number());

                log_println!("  Firmware Revision: {:x?}", descriptor.firmware_revision);

                log_println!("  Model Number: {:?}", descriptor.model_number());

                log_print!("  UDMA available modes: ");

                for (i, mode) in descriptor.udma_available_modes.iter().enumerate() {
                    if *mode {
                        log_print!("{} ", i + 1);
                    }
                }
                log_println!();
                log_println!(
                    "  UDMA active mode: UDMA{}",
                    8 - descriptor.udma_current_mode.leading_zeros()
                );

                log_println!(
                    "  Supported LBA 28 sectors: {}",
                    descriptor.lba_28_addressable_sectors
                );
                log_println!("  Supports LBA 48: {}", descriptor.supports_lba_48);
                log_println!(
                    "  Supported LBA 48 sectors: {}",
                    descriptor.lba_48_addressable_sectors
                );
            }
            Err(error) => {
                log_println!("{}:ATA device error: {:?}", index, error);
            }
        }
    }
    */

    let test = Box::new(4);
    log_println!("New boxed value: {:#?}", test);
    log_println!("im not dying :)");

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&kernel_info.memory_regions) };
    let mut memory_mapper = MEMORY_MAPPER.lock();

    // should map virtual address 4444_0000_0000 to physical address 4444_4444_0000
    let page = Page::containing_address(VirtAddr::new(0x4444_0000_0000));
    page_table::create_mapping(
        page,
        0x4444_4444_0000,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        memory_mapper.as_mut().unwrap(),
        &mut frame_allocator,
    );

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    unsafe {
        page_ptr.offset(300).write_volatile(42);
        log_println!("Did this work= {}", page_ptr.offset(300).read_volatile())
    }
}

/// Panic handler for the OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::log_print!("{}", info);
    os_core::hlt_loop();
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
    os_core::test_panic_handler(info);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
