#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info)
}

entry_point!(kernel_start);
#[no_mangle]
pub fn kernel_start(_boot_info: &'static mut BootInfo) -> ! {
    os_core::init(_boot_info);
    test_main();
    loop {}
}

#[test_case]
pub fn interrupt_test() {
    x86_64::instructions::interrupts::int3();
    // if execution reaches this point, the test has passed
}
