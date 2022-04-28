use alloc::vec::Vec;
use ata::{ATADisk, ATAPartition, PartitionIOError};

use super::superblock::Superblock;

const EXT2_PARTITION_ID: u8 = 0x83;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemError {
    PartitionIOError(PartitionIOError),
}

pub fn get_file_system_partitions() -> Vec<ATAPartition> {
    ata::get_all_disks()
        .into_iter()
        .filter_map(|mut disk| disk.get_partitions().ok())
        .flat_map(|p| {
            p.into_iter()
                .filter(|partition| partition.descriptor.file_system == EXT2_PARTITION_ID)
        })
        .collect()
}

pub fn get_all_disks() -> Vec<ATADisk> {
    ata::get_all_disks()
}

pub fn create_and_init_partition(
    disk: &mut ATADisk,
    bytes: u64,
) -> Result<ATAPartition, FileSystemError> {
    let mut partition = disk
        .create_partition(((bytes + 511) / 512) as u32, EXT2_PARTITION_ID)
        .map_err(|err| FileSystemError::PartitionIOError(err))?;
    init_partition(&mut partition)?;
    Ok(partition)
}

pub fn init_partition(partition: &mut ATAPartition) -> Result<(), FileSystemError> {
    Ok(())
}

pub fn read_superblock(partition: &mut ATAPartition) -> Result<Superblock, FileSystemError> {
    let mut superblock = [0u8; 1024];
    let slice1 = partition
        .read_sector(2)
        .map_err(|err| FileSystemError::PartitionIOError(err))?;
    let slice2 = partition
        .read_sector(3)
        .map_err(|err| FileSystemError::PartitionIOError(err))?;
    superblock[0..512].copy_from_slice(&slice1);
    superblock[512..1024].copy_from_slice(&slice2);
    Ok(Superblock::from_bytes(&superblock))
}
