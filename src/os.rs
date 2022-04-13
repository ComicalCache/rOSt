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

use core::panic::PanicInfo;

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    // initialisation steps of the OS
    // ! DO NOT REMOVE!
    os_core::init();

    // `cargo test` entry point.
    // ! DO NOT REMOVE!
    #[cfg(test)]
    test_main();

    // ########################################
    // # _start() actual entry on `cargo run` #
    // ########################################

    // this causes a panic and the OS will handle it
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    loop {}
}

// ################################################################
// #   Defines panic handlers for testing and regular execution   #
// ################################################################

#[cfg(not(test))]
#[panic_handler]
// this function is called if a panic occurs and is not a test, all output is redirected to the VGA buffer
fn panic(info: &PanicInfo) -> ! {
    use os_core::{interface::VGA_TEXT_BUFFER_INTERFACE, print};

    // needs to force unlock the VgaTextBufferInterface because it could be locked when the panic occurs
    // this should be safe because the panic handler is the last thing that is executed
    unsafe { VGA_TEXT_BUFFER_INTERFACE.force_unlock() };
    VGA_TEXT_BUFFER_INTERFACE.lock().__set_panic_config();

    print!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    use os_core::test_panic_handler;

    test_panic_handler(info)
}
