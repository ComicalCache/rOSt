#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DirectoryEntryType {
    Unknown = 0,
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    SymbolicLink,
}

pub struct DirectoryEntry {
    inode: u32,
    entry_total_size: u16,
    low_name_length: u8,
    type_or_high_name_length: DirectoryEntryType,
    name: [u8],
}
