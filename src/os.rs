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

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::structures::kernel_information::KernelInformation;

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
    let test = Box::new(4);
    log_println!("New boxed value: {:#?}", test);
    log_println!("im not dying :)");
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
