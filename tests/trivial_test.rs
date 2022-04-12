#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_testing::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_testing::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
pub fn trivial_assertion_true() {
    assert_eq!(1, 1);
}