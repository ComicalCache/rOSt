use alloc::vec::Vec;
use ata::ATAPartition;

use self::{
    blockgroup_descriptor_table::BlockGroupDescriptor, operations::FileSystemError,
    superblock::Superblock,
};

pub mod blockgroup_descriptor_table;
pub mod inode;
pub mod operations;
pub mod superblock;

pub struct Ext2Driver {
    partition: ATAPartition,
    superblock: Superblock,
    blockgroup_descriptor_table: Vec<BlockGroupDescriptor>,
}

impl Ext2Driver {
    pub fn new(partition: &ATAPartition) -> Result<Self, FileSystemError> {
        let mut partition = partition.clone();
        let superblock = operations::read_superblock(&mut partition)?;
        let group_count =
            (superblock.total_blocks + superblock.blocks_in_group - 1) / superblock.blocks_in_group;
        let sectors_to_load = ((group_count + 15) / 16) as u64;
        let mut blockgroup_descriptor_table = Vec::new();
        for i in 0..sectors_to_load {
            let sector = partition
                .read_sector(4 + i)
                .map_err(|err| FileSystemError::PartitionIOError(err))?;
            for chunk in sector.chunks(32) {
                blockgroup_descriptor_table.push(BlockGroupDescriptor::from_bytes(chunk));
            }
        }

        Ok(Self {
            partition,
            superblock,
            blockgroup_descriptor_table,
        })
    }

    pub fn 
}
