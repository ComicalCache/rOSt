#![no_std] // no standard library
#![no_main]
use os_core::structures::{driver::Driver, kernel_information::KernelInformation};
extern crate alloc;

pub mod bus;
pub mod constants;
pub mod descriptor;

pub extern "C" fn driver_init(_kernel_info: KernelInformation) -> Driver {
    Driver {
        signature: [
            0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1,
            0xf2, 0xf3,
        ],
    }
}
