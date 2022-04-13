#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os_core::{interface::VGA_TEXT_BUFFER_INTERFACE, println};

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
pub fn simple_print_test() {
    println!("Hello, World!");
}

#[test_case]
pub fn bulk_print_test() {
    for _ in 0..250 {
        println!("Hello, World!");
    }
}

#[test_case]
pub fn read_byte() {
    VGA_TEXT_BUFFER_INTERFACE.lock().set_pos(0, 0);
    println!("Hello, World!");
    assert_eq!(b'H', VGA_TEXT_BUFFER_INTERFACE.lock().read_byte(0, 0));
    assert_eq!(b',', VGA_TEXT_BUFFER_INTERFACE.lock().read_byte(0, 5));
}
