use internal_utils::{format_size, serial_println};

use crate::{ATABus, ATADisk, ATAIdentifyError};

pub fn debug_disks() {
    let disk_a = ATABus::get_disk(&*crate::PRIMARY_ATA_BUS, true);
    let disk_b = ATABus::get_disk(&*crate::PRIMARY_ATA_BUS, false);
    let disk_c = ATABus::get_disk(&*crate::SECONDARY_ATA_BUS, true);
    let disk_d = ATABus::get_disk(&*crate::SECONDARY_ATA_BUS, false);
    serial_println!("[   ---{:^15}---   ]", "DISKS");
    debug_disk(disk_a, "Primary 1");
    debug_disk(disk_b, "Primary 2");
    debug_disk(disk_c, "Secondary 1");
    debug_disk(disk_d, "Secondary 2");
}

fn debug_disk(disk: Result<ATADisk, ATAIdentifyError>, disk_type: &str) {
    if let Ok(disk) = disk {
        serial_println!(
            "[{:^11}] {:<20}: {} ({} partitions){}",
            disk_type,
            disk.descriptor.model_number().trim(),
            format_size(disk.descriptor.lba_48_addressable_sectors * 512),
            disk.clone().get_partitions().map(|p| p.len()).unwrap_or(0),
            disk.clone()
                .has_bootloader()
                .map(|b| if b { " (has bootloader)" } else { "" })
                .unwrap_or(", Error while reading start sector")
        );
    }
}
