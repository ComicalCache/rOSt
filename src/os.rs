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
// no entry point

// #################################################
// #   This produces a runnable binary of the OS   #
// #################################################

extern crate alloc;

use alloc::boxed::Box;
use core::alloc::Layout;
use core::panic::PanicInfo;

use bootloader::{
    boot_info::{FrameBuffer, Optional},
    BootInfo, entry_point,
};

use os_core::{hlt_loop, log_println};
use os_core::basic_drivers::vga::vga_buffer::VGADeviceFactory;

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    kernel_test(boot_info);
    #[cfg(not(test))]
    kernel_main(boot_info);
    hlt_loop();
}

/// This is the **main function** of the OS when not in test mode.
pub fn kernel_main(boot_info: &'static mut BootInfo) {
    // ? once we have a proper writer (should be instantiated in the os_core::init function) we should outsource
    // ? the os_core::init call from kernel_main and kernel_test to the general kernel function since it will
    let framebuffer_pointer: *mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();
    let os_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };
    if let Optional::Some(physical_memory_offset) = boot_info.physical_memory_offset {
        os_core::init(os_framebuffer, physical_memory_offset, &boot_info.memory_regions);
    } else {
        panic!("Error retaining the physical memory offset");
    }
    let usable_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };

    let mut _device = VGADeviceFactory::from_buffer(usable_framebuffer);

    let test = Box::new(4);
    log_println!("New boxed value: {:#?}", test);
    log_println!("im not dying :)");
}

/// Panic handler for the OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::log_print!("{}", info);
    hlt_loop();
}

/// This is the main function for tests.
#[cfg(test)]
pub fn kernel_test(boot_info: &'static mut BootInfo) {
    if let Optional::Some(physical_memory_offset) = boot_info.physical_memory_offset {
        os_core::init(boot_info.framebuffer.as_mut().unwrap(), physical_memory_offset, &boot_info.memory_regions);
    } else {
        panic!("Error retaining the physical memory offset");
    }
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
