#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use alloc::vec::Vec;
use kernel::structures::{driver::Driver, kernel_information::KernelInformation};
extern crate alloc;

mod constants;
pub use constants::{ATAIdentifyError, PRIMARY_ATA_BUS, SECONDARY_ATA_BUS};

mod bus;
pub use bus::ATABus;

mod disk_descriptor;
pub use disk_descriptor::DiskDescriptor;

mod partition_descriptor;
pub use partition_descriptor::PartitionDescriptor;

mod disk;
pub use disk::ATADisk;

mod partition;
pub use partition::{ATAPartition, PartitionIOError};

#[cfg(debug_assertions)]
mod debug;

pub extern "C" fn driver_init(_kernel_info: KernelInformation) -> Driver {
    #[cfg(debug_assertions)]
    debug::debug_disks();
    Driver {
        signature: [
            0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1, 0xf2, 0xf3, 0xf0, 0xf1,
            0xf2, 0xf3,
        ],
    }
}

pub fn get_all_disks() -> Vec<ATADisk> {
    [
        (&*PRIMARY_ATA_BUS, true),
        (&*PRIMARY_ATA_BUS, false),
        (&*SECONDARY_ATA_BUS, true),
        (&*SECONDARY_ATA_BUS, false),
    ]
    .iter()
    .filter_map(|(bus, master)| ATABus::get_disk(bus, *master).ok())
    .collect()
}
