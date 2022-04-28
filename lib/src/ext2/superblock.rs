use bitflags::bitflags;
use utils::byte_reader::ByteReader;

bitflags! {
    pub struct OptionalFeatures: u32 {
        const DIRECTORIES_USE_HASH_INDEX = 0x0020;
        const FILESYSTEM_CAN_RESIZE = 0x0010;
        const INODES_HAVE_EXTENDED_ATTRIBUTES = 0x0008;
        const FILESYSTEM_HAS_A_JOURNAL = 0x0004;
        const AFS_INODES_EXIST = 0x0002;
        const PREALLOCATE_BLOCKS = 0x0001;
    }

    pub struct RequiredFeatures: u32 {
        const FILESYSTEM_USES_A_JOURNAL_DEVICE = 0x0008;
        const FILESYSTEM_NEEDS_TO_REPLAY_JOURNAL = 0x0004;
        const DIRECTORIES_HAVE_A_TYPE_FIELD = 0x0002;
        const COMPRESSION_USED = 0x0001;
    }

    pub struct ReadOnlyFeatures: u32 {
        const DIRECTORIES_STORED_IN_BINARY_TREE = 0x0004;
        const FILESYSTEM_USES_64_BIT_FILE_SIZE = 0x0002;
        const SPARSE_SUPERBLOCKS = 0x0001;
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileSystemState {
    Clean = 1,
    HasErrors = 2,
}

impl From<u16> for FileSystemState {
    fn from(value: u16) -> Self {
        match value {
            1 => FileSystemState::Clean,
            2 => FileSystemState::HasErrors,
            _ => unreachable!(),
        }
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorHandlingStrategy {
    Ignore = 1,
    RemountAsReadonly = 2,
    Panic = 3,
}

impl From<u16> for ErrorHandlingStrategy {
    fn from(value: u16) -> Self {
        match value {
            1 => ErrorHandlingStrategy::Ignore,
            2 => ErrorHandlingStrategy::RemountAsReadonly,
            3 => ErrorHandlingStrategy::Panic,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatorOperatingSystem {
    Linux = 0,
    GNUHurd = 1,
    Masix = 2,
    FreeBSD = 3,
    Lites = 4,
}

impl From<u32> for CreatorOperatingSystem {
    fn from(value: u32) -> Self {
        match value {
            0 => CreatorOperatingSystem::Linux,
            1 => CreatorOperatingSystem::GNUHurd,
            2 => CreatorOperatingSystem::Masix,
            3 => CreatorOperatingSystem::FreeBSD,
            4 => CreatorOperatingSystem::Lites,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Superblock {
    pub total_inodes: u32,
    pub total_blocks: u32,
    pub superuser_blocks: u32,
    pub unallocated_blocks: u32,
    pub unallocated_inodes: u32,
    pub container_block_id: u32,
    pub block_size_modifier: u32,
    pub fragment_size_modifier: u32,
    pub blocks_in_group: u32,
    pub fragments_in_group: u32,
    pub inodes_in_group: u32,
    pub last_mount_time: u32,
    pub last_write_time: u32,
    pub mount_count: u16,
    pub max_mount_count: u16,
    pub signature: u16,
    pub file_system_state: FileSystemState,
    pub error_handling: ErrorHandlingStrategy,
    pub version_minor: u16,
    pub last_consistency_check_time: u32,
    pub forces_consistency_check_interval: u32,
    pub operating_system_id: CreatorOperatingSystem,
    pub version_major: u32,
    pub superuser_user_id: u16,
    pub superuser_group_id: u16,
    // EXT2 Additional fields
    pub first_non_reserved_inode: u32,
    pub inode_structure_size: u16,
    pub blockgroup_id: u16,
    pub optional_features: OptionalFeatures,
    pub required_features: RequiredFeatures,
    pub required_readonly_features: ReadOnlyFeatures,
    pub file_system_id: [u8; 16],
    pub volume_name: [u8; 16],
    pub path_volume: [u8; 64],
    pub compression_algorithm: u32,
    pub blocks_to_preallocate_for_files: u8,
    pub blocks_to_preallocate_for_directories: u8,
    pub unused: u16,
    pub journal_id: [u8; 16],
    pub journal_inode: u32,
    pub journal_device: u32,
    pub orphan_inode_list_head: u32,
    pub unused_2: [u8; 788],
}

impl Superblock {
    pub(super) fn from_bytes(data: &[u8; 1024]) -> Self {
        let mut reader = ByteReader::of(data);
        Self {
            total_inodes: reader.read_u32(),
            total_blocks: reader.read_u32(),
            superuser_blocks: reader.read_u32(),
            unallocated_blocks: reader.read_u32(),
            unallocated_inodes: reader.read_u32(),
            container_block_id: reader.read_u32(),
            block_size_modifier: reader.read_u32(),
            fragment_size_modifier: reader.read_u32(),
            blocks_in_group: reader.read_u32(),
            fragments_in_group: reader.read_u32(),
            inodes_in_group: reader.read_u32(),
            last_mount_time: reader.read_u32(),
            last_write_time: reader.read_u32(),
            mount_count: reader.read_u16(),
            max_mount_count: reader.read_u16(),
            signature: reader.read_u16(),
            file_system_state: reader.read_enum_u16::<FileSystemState>(),
            error_handling: reader.read_enum_u16::<ErrorHandlingStrategy>(),
            version_minor: reader.read_u16(),
            last_consistency_check_time: reader.read_u32(),
            forces_consistency_check_interval: reader.read_u32(),
            operating_system_id: reader.read_enum_u32::<CreatorOperatingSystem>(),
            version_major: reader.read_u32(),
            superuser_user_id: reader.read_u16(),
            superuser_group_id: reader.read_u16(),
            first_non_reserved_inode: reader.read_u32(),
            inode_structure_size: reader.read_u16(),
            blockgroup_id: reader.read_u16(),
            optional_features: OptionalFeatures::from_bits_truncate(reader.read_u32()),
            required_features: RequiredFeatures::from_bits_truncate(reader.read_u32()),
            required_readonly_features: ReadOnlyFeatures::from_bits_truncate(reader.read_u32()),
            file_system_id: reader.read_slice::<16>(),
            volume_name: reader.read_slice::<16>(),
            path_volume: reader.read_slice::<64>(),
            compression_algorithm: reader.read_u32(),
            blocks_to_preallocate_for_files: reader.read_u8(),
            blocks_to_preallocate_for_directories: reader.read_u8(),
            unused: reader.read_u16(),
            journal_id: reader.read_slice::<16>(),
            journal_inode: reader.read_u32(),
            journal_device: reader.read_u32(),
            orphan_inode_list_head: reader.read_u32(),
            unused_2: reader.read_slice::<788>(),
        }
    }
}
