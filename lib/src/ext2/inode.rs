use bitflags::bitflags;
use utils::byte_reader::ByteReader;

bitflags! {
  #[repr(C)]
  pub struct TypeAndPermissions: u16 {
    //Types
    const UNIX_SOCKET = 0xC000;
    const SYMBOLIC_LINK = 0xA000;
    const FILE = 0x8000;
    const BLOCK_DEVICE = 0x6000;
    const DIRECTORY = 0x4000;
    const CHARACTER_DEVICE = 0x2000;
    const FIFO = 0x1000;
    //Permissions
    const SET_USER_ID = 0x800;
    const SET_GROUP_ID = 0x400;
    const STICKY = 0x200;
    const USER_READ = 0x100;
    const USER_WRITE = 0x80;
    const USER_EXECUTE = 0x40;
    const GROUP_READ = 0x20;
    const GROUP_WRITE = 0x10;
    const GROUP_EXECUTE = 0x8;
    const OTHER_READ = 0x4;
    const OTHER_WRITE = 0x2;
    const OTHER_EXECUTE = 0x1;
  }

  #[repr(C)]
  pub struct INodeFlags: u32 {
    const JOURNAL_FILE_DATA = 0x00040000;
    const AFS_DIRECTORY = 0x00020000;
    const HASH_INDEXED_DIRECTORY = 0x00010000;
    // Reserved
    const DO_NOT_UPDATE_LAST_ACCESS_TIME = 0x80;
    const FILE_NOT_INCLUDED_IN_DUMP = 0x40;
    const APPEND_ONLY = 0x20;
    const IMMUTABLE_FILE = 0x10;
    const SYNCHRONOUS = 0x08;
    // Not Used
  }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct INode {
    pub type_and_permissions: TypeAndPermissions,
    pub user_id: u16,
    pub low_size: u32,
    pub last_access_time: u32,
    pub creation_time: u32,
    pub last_modification_time: u32,
    pub deletion_time: u32,
    pub group_id: u16,
    pub hard_links: u16,
    pub disk_sectors_used: u32,
    pub flags: u32,
    pub os_specific_value_1: u32,
    pub direct_block_pointer_0: u32,
    pub direct_block_pointer_1: u32,
    pub direct_block_pointer_2: u32,
    pub direct_block_pointer_3: u32,
    pub direct_block_pointer_4: u32,
    pub direct_block_pointer_5: u32,
    pub direct_block_pointer_6: u32,
    pub direct_block_pointer_7: u32,
    pub direct_block_pointer_8: u32,
    pub direct_block_pointer_9: u32,
    pub direct_block_pointer_10: u32,
    pub direct_block_pointer_11: u32,
    pub singly_indirect_block_pointer: u32,
    pub doubly_indirect_block_pointer: u32,
    pub triply_indirect_block_pointer: u32,
    pub generation_number: u32,
    pub extended_attribute_block: u32,
    pub high_size_or_directory_acl: u32,
    pub fragment_block_address: u32,
    pub fragment_number: u8,
    pub fragment_size: u8,
}

impl INode {
    pub fn from_bytes(bytes: &[u8; 128]) -> Self {
        let mut reader = ByteReader::of(bytes);
        Self {
            type_and_permissions: TypeAndPermissions::from_bits_truncate(reader.read_u16()),
            user_id: reader.read_u16(),
            low_size: reader.read_u32(),
            last_access_time: reader.read_u32(),
            creation_time: reader.read_u32(),
            last_modification_time: reader.read_u32(),
            deletion_time: reader.read_u32(),
            group_id: reader.read_u16(),
            hard_links: reader.read_u16(),
            disk_sectors_used: reader.read_u32(),
            flags: reader.read_u32(),
            os_specific_value_1: reader.read_u32(),
            direct_block_pointer_0: reader.read_u32(),
            direct_block_pointer_1: reader.read_u32(),
            direct_block_pointer_2: reader.read_u32(),
            direct_block_pointer_3: reader.read_u32(),
            direct_block_pointer_4: reader.read_u32(),
            direct_block_pointer_5: reader.read_u32(),
            direct_block_pointer_6: reader.read_u32(),
            direct_block_pointer_7: reader.read_u32(),
            direct_block_pointer_8: reader.read_u32(),
            direct_block_pointer_9: reader.read_u32(),
            direct_block_pointer_10: reader.read_u32(),
            direct_block_pointer_11: reader.read_u32(),
            singly_indirect_block_pointer: reader.read_u32(),
            doubly_indirect_block_pointer: reader.read_u32(),
            triply_indirect_block_pointer: reader.read_u32(),
            generation_number: reader.read_u32(),
            extended_attribute_block: reader.read_u32(),
            high_size_or_directory_acl: reader.read_u32(),
            fragment_block_address: reader.read_u32(),
            fragment_number: reader.read_u8(),
            fragment_size: reader.read_u8(),
        }
    }
}
