use crate::{constants::ErrorRegisterFlags, ATADisk, DiskDescriptor, PartitionDescriptor};

#[derive(Clone)]
pub struct ATAPartition {
    pub(crate) disk: ATADisk,
    pub descriptor: PartitionDescriptor,
}

impl ATAPartition {
    pub fn disk_descriptor(&self) -> DiskDescriptor {
        self.disk.descriptor.clone()
    }

    pub fn read_sector(&mut self, lba: u64) -> Result<[u8; 512], PartitionIOError> {
        if lba >= self.descriptor.sectors {
            Err(PartitionIOError::AddressNotInRange)
        } else {
            self.disk
                .read_sector(lba + self.descriptor.start_lba)
                .map_err(|err| PartitionIOError::ATAError(err))
        }
    }

    pub fn write_sector(&mut self, lba: u64, buffer: &[u8; 512]) -> Result<(), PartitionIOError> {
        if lba >= self.descriptor.sectors {
            Err(PartitionIOError::AddressNotInRange)
        } else {
            self.disk
                .write_sector(lba + self.descriptor.start_lba, buffer)
                .map_err(|err| PartitionIOError::ATAError(err))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionIOError {
    ATAError(ErrorRegisterFlags),
    AddressNotInRange,
    TooManyPartitions,
    Unknown,
}
