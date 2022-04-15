#![no_std] // no standard library
#![no_main]
// no entry point

// #################################################
// #   This produces a runnable binary of the OS   #
// #################################################

// configures our test framework environment
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os_core::vga::vga_core::{TextDrawable, PlainDrawable};
use bootloader::{entry_point, BootInfo, boot_info::FrameBuffer};
use os_core::vga::{vga_buffer::VGADeviceFactory, vga_core::Clearable, vga_color};
use core::panic::PanicInfo;

use os_core::hlt_loop;

entry_point!(kernel_start);
#[no_mangle]
pub fn kernel_start(_boot_info: &'static mut BootInfo) -> ! {
    let framebuffer_pointer: *mut FrameBuffer = _boot_info.framebuffer.as_mut().unwrap();

    let os_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };
    os_core::init(os_framebuffer);

    #[cfg(test)]
    test_main();

    let usable_framebuffer = unsafe { framebuffer_pointer.as_mut().unwrap() };


    let mut device = VGADeviceFactory::from_buffer(usable_framebuffer);
    device.clear(&vga_color::BLACK);
    device.draw_string(10, 10, &vga_color::WHITE, "Hello, world!", 0);
    device.fill_rectangle(100, 50, 50, 70, &vga_color::GREEN);
    device.draw_rectangle(0, 0, 250, 270, &vga_color::RED);

    // this causes a panic and the OS will handle it
    
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    

    hlt_loop();
}

// ################################################################
// #   Defines panic handlers for testing and regular execution   #
// ################################################################

#[cfg(not(test))]
#[panic_handler]
// this function is called if a panic occurs and is not a test, all output is redirected to the VGA buffer
fn panic(info: &PanicInfo) -> ! {
    use os_core::print;

    // this should be safe because the panic handler is the last thing that is executed

    print!("{}", info);

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    use os_core::test_panic_handler;

    test_panic_handler(info)
}
