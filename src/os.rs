#![no_std] // no standard library
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt, generic_const_exprs, core_intrinsics)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]
// no entry point

// #################################################
// #   This produces a runnable binary of the OS   #
// #################################################

use os_core::vga::{vga_core::{TextDrawable, PlainDrawable, ShapeDrawable}, point_2d::Point2D};
use bootloader::{BootInfo, boot_info::FrameBuffer, entry_point};
use os_core::vga::{vga_buffer::VGADeviceFactory, vga_core::Clearable, vga_color};
use core::panic::PanicInfo;

use os_core::hlt_loop;

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    //#[cfg(test)]
    //kernel_test(boot_info);
    //#[cfg(not(test))]
    kernel_main(boot_info);
    hlt_loop();
}

pub fn kernel_main(boot_info: &'static mut BootInfo) {
    
    let framebuffer_pointer: *mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();

    let os_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };
    os_core::init(os_framebuffer);

    let usable_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };


    let mut device = VGADeviceFactory::from_buffer(usable_framebuffer);
    device.clear(&vga_color::BLACK);
    device.draw_string(10, 10, &vga_color::WHITE, "Hello, world!", 0);
    device.fill_rectangle(100, 50, 50, 70, &vga_color::GREEN);
    device.draw_rectangle(0, 0, 250, 270, &vga_color::RED);
    //device.draw_bezier(
    //    &VGAPoint { x: 0, y: 0},
    //    &VGAPoint { x: 250, y: 0},
    //    &VGAPoint { x: 250, y: 270},
    //    &VGAPoint { x: 0, y: 270},
    //    &vga_color::WHITE
    //);
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
    os_core::print!("{}", info);
    hlt_loop();
}

#[cfg(test)]
pub fn kernel_test(boot_info: &'static mut BootInfo) {
    os_core::init(boot_info.framebuffer.as_mut().unwrap());
    test_main();
}

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info);
}
