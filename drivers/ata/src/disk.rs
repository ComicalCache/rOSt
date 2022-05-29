use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;
use utils::array_combiner::Combiner;

use crate::{
    constants::ErrorRegisterFlags, ATABus, ATAPartition, DiskDescriptor, PartitionDescriptor,
    PartitionIOError,
};

#[derive(Clone)]
pub struct ATADisk {
    pub(crate) bus: Arc<Mutex<ATABus>>,
    pub descriptor: DiskDescriptor,
    pub(crate) master: bool,
}

impl ATADisk {
    pub fn has_bootloader(&mut self) -> Result<bool, ErrorRegisterFlags> {
        let buffer = self.read_sector(0)?;
        Ok(buffer[510] == 0x55 && buffer[511] == 0xAA)
    }

    pub(crate) fn read_sector(&mut self, lba: u64) -> Result<[u8; 512], ErrorRegisterFlags> {
        self.bus.lock().read_sector(self.master, lba)
    }

    pub(crate) fn write_sector(
        &mut self,
        lba: u64,
        buffer: &[u8; 512],
    ) -> Result<(), ErrorRegisterFlags> {
        self.bus.lock().write_sector(self.master, lba, buffer)
    }

    pub fn get_partitions(&mut self) -> Result<Vec<ATAPartition>, ErrorRegisterFlags> {
        let mbr = self.read_sector(0)?;
        let descriptors = mbr[446..510]
            .chunks(16)
            .filter_map(PartitionDescriptor::from_bytes);
        let mut partitions = Vec::new();
        for descriptor in descriptors {
            partitions.push(ATAPartition {
                disk: self.clone(),
                descriptor,
            });
        }
        Ok(partitions)
    }

    pub fn create_partition(
        &mut self,
        sectors: u32,
        partition_type: u8,
    ) -> Result<ATAPartition, PartitionIOError> {
        let mut mbr = self.read_sector(0).map_err(PartitionIOError::ATAError)?;
        let descriptors: Vec<PartitionDescriptor> = mbr[446..510]
            .chunks(16)
            .filter_map(PartitionDescriptor::from_bytes)
            .collect();
        if descriptors.len() >= 4 {
            return Err(PartitionIOError::TooManyPartitions);
        }

        let start_sector_bytes = {
            let start_sector = (descriptors
                .iter()
                .map(|d| d.start_lba + d.sectors)
                .max()
                .unwrap_or(0)
                + 1) as u32;
            u32::to_le_bytes(start_sector)
        };

        let sectors_bytes = u32::to_le_bytes(sectors);

        let partition_bytes = Combiner::new()
            .with(&[0x00, 0xFF, 0xFF, 0xFF])
            .with(&[partition_type, 0xFF, 0xFF, 0xFF])
            .with(&start_sector_bytes)
            .with(&sectors_bytes)
            .build::<16>()
            .expect("Wrong number of bytes for a partition descriptor");

        let descriptor = PartitionDescriptor::from_bytes(&partition_bytes);
        if descriptor.is_none() {
            return Err(PartitionIOError::Unknown);
        }
        let descriptor = descriptor.unwrap();
        let partition_free_index = mbr[446..510]
            .chunks(16)
            .enumerate()
            .map(|(index, val)| (PartitionDescriptor::from_bytes(val), index))
            .find(|(val, _)| val.is_none())
            .map(|(_, i)| i);
        if partition_free_index.is_none() {
            return Err(PartitionIOError::Unknown);
        }
        let partition_free_index = partition_free_index.unwrap();
        mbr[446 + partition_free_index * 16..446 + partition_free_index * 16 + 16]
            .copy_from_slice(&partition_bytes);
        self.write_sector(0, &mbr)
            .map_err(PartitionIOError::ATAError)?;
        Ok(ATAPartition {
            disk: self.clone(),
            descriptor,
        })
    }
}
