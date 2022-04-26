#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PartitionDescriptor {
    pub bootable: bool,
    pub start_lba: u64,
    pub sectors: u64,
}

impl PartitionDescriptor {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<PartitionDescriptor> {
        if bytes.iter().all(|b| *b == 0x00) {
            return None;
        }
        Some(PartitionDescriptor {
            bootable: bytes[0] == 0x80,
            start_lba: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as u64,
            sectors: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as u64,
        })
    }
}
