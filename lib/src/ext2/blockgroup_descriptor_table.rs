use utils::byte_reader::ByteReader;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockGroupDescriptor {
    block_usage_bitmap_address: u32,
    inode_usage_bitmap_address: u32,
    inode_table_address: u32,
    unallocated_blocks: u16,
    unallocated_inodes: u16,
    directories: u16,
    unused: [u8; 14],
}

impl BlockGroupDescriptor {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut reader = ByteReader::of(bytes);
        Self {
            block_usage_bitmap_address: reader.read_u32(),
            inode_usage_bitmap_address: reader.read_u32(),
            inode_table_address: reader.read_u32(),
            unallocated_blocks: reader.read_u16(),
            unallocated_inodes: reader.read_u16(),
            directories: reader.read_u16(),
            unused: reader.read_slice::<14>(),
        }
    }
}
