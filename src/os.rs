#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    generic_const_exprs,
    core_intrinsics
)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ata::constants::{PRIMARY_ATA_BUS, SECONDARY_ATA_BUS};
use bootloader::{boot_info::FrameBuffer, entry_point, BootInfo};
use core::panic::PanicInfo;
use os_core::structures::kernel_information::KernelInformation;

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    os_core::register_driver(vga::driver_init);
    os_core::register_driver(ata::driver_init);
    let kernel_info = os_core::init(boot_info);

    #[cfg(test)]
    kernel_test(kernel_info);
    #[cfg(not(test))]
    os_core::kernel_main(kernel_info);

    os_core::hlt_loop();
}

pub fn kernel_main(kernel_info: KernelInformation) {
    // ? once we have a proper writer (should be instanciated in the os_core::init function) we should outsource
    // ? the os_core::init call from kernel_main and kernel_test to the general kernel function since it will
    let framebuffer_pointer: *mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();

    let mut device = VGADeviceFactory::from_buffer(usable_framebuffer);
    device.clear(vga_color::BLACK);

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

    device.draw_string(350, 300, vga_color::WHITE, "Hello, world!", 0);
    device.fill_rectangle(350, 350, 50, 70, vga_color::GREEN);
    device.draw_rectangle(300, 250, 250, 200, vga_color::RED);
    device.draw_bezier(
        Point2D { x: 300, y: 250 },
        Point2D { x: 550, y: 250 },
        Point2D { x: 550, y: 450 },
        Point2D { x: 300, y: 450 },
        vga_color::WHITE,
    );

    // this causes a panic and the OS will handle it
    /*
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    */
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::log_print!("{}", info);
    hlt_loop();
}

#[cfg(test)]
pub fn kernel_test(kernel_info: KernelInformation) {
    test_main();
}

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info);
}
