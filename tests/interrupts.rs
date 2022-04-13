#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os_core::init();
    test_main();
    loop {}
}

#[test_case]
pub fn intterrupt_test() {
    x86_64::instructions::interrupts::int3();
    // if execution reaches this point, the test has passed
}
