#[derive(Debug, PartialEq, Eq)]
pub struct Partition {
    pub bootable: bool,
    pub start_lba: u32,
    pub sectors: u32,
}

impl Partition {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Partition> {
        if bytes.iter().all(|b| *b == 0x00) {
            return None;
        }
        Some(Partition {
            bootable: bytes[0] == 0x80,
            start_lba: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            sectors: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        })
    }
}
