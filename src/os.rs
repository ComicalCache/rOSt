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
// no entry point

// #################################################
// #   This produces a runnable binary of the OS   #
// #################################################

use bootloader::{
    boot_info::{FrameBuffer, Optional},
    entry_point, BootInfo,
};
use core::panic::PanicInfo;
use x86_64::{structures::paging::Translate, VirtAddr};

/*
use os_core::vga::point_2d::Point2D;
use os_core::vga::vga_core::{PlainDrawable, ShapeDrawable, TextDrawable};
use os_core::vga::vga_color, vga_core::Clearable};
*/
use os_core::memory::page_table::init;
use os_core::vga::vga_buffer::VGADeviceFactory;

use os_core::{hlt_loop, log_println};

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
    // ? once we have a proper writer (should be instanciated in the os_core::init function) we should outsource
    // ? the os_core::init call from kernel_main and kernel_test to the general kernel function since it will
    let framebuffer_pointer: *mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();
    let os_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };
    os_core::init(os_framebuffer);
    let usable_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };
    let mut _device = VGADeviceFactory::from_buffer(usable_framebuffer);

    if let Optional::Some(physical_memory_offset) = boot_info.physical_memory_offset {
        let phys_mem_offset = VirtAddr::new(physical_memory_offset);
        let mapper = unsafe { init(phys_mem_offset) };

        let addresses = [
            // the identity-mapped vga buffer page
            0xb8000,
            // some code page
            0x201008,
            // some stack page
            0x0100_0020_1a10,
            // virtual address mapped to physical address 0
            physical_memory_offset,
        ];

        for &address in &addresses {
            let virt = VirtAddr::new(address);
            // new: use the `mapper.translate_addr` method
            let phys = mapper.translate_addr(virt);
            log_println!("{:?} -> {:?}", virt, phys);
        }
    }
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
    os_core::init(boot_info.framebuffer.as_mut().unwrap());
    test_main();
}

/// Panic handler for the OS in test mode.
#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info);
}
