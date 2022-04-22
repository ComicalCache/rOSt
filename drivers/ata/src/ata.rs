#![no_std] // no standard library
#![no_main]
use kernel::structures::{driver::Driver, kernel_information::KernelInformation};
extern crate alloc;

mod constants;
pub use constants::{ATAIdentifyError, PRIMARY_ATA_BUS, SECONDARY_ATA_BUS};

mod bus;
pub use bus::ATABus;

mod descriptor;
pub use descriptor::ATADescriptor;

mod partition;
pub use partition::Partition;

pub extern "C" fn driver_init(_kernel_info: KernelInformation) -> Driver {
    Driver {
        signature: [
            0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1,
            0xf2, 0xf3,
        ],
    }
}
