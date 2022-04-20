use spin::Mutex;

use super::bus::ATABus;

pub mod status_register_flags {
    /// Busy
    pub const BSY: u8 = 0x80;
    /// Device Ready
    pub const DRDY: u8 = 0x40;
    /// Device Fault
    pub const DF: u8 = 0x20;
    /// Seek Complete
    pub const DSC: u8 = 0x10;
    /// Data Transfer Required
    pub const DRQ: u8 = 0x08;
    /// Data Corrected
    pub const CORR: u8 = 0x04;
    /// Index Mark
    pub const IDX: u8 = 0x02;
    /// Error
    pub const ERR: u8 = 0x01;
}

pub mod error_register_flags {
    /// Bad Block
    pub const BBK: u8 = 0x80;
    /// Uncorrectable Data Error
    pub const UNC: u8 = 0x40;
    /// Media Changed
    pub const MC: u8 = 0x20;
    /// ID Mark Not Found
    pub const IDNF: u8 = 0x10;
    /// Media Change Requested
    pub const MCR: u8 = 0x08;
    /// Command Aborted
    pub const ABRT: u8 = 0x04;
    /// Track 0 Not Found
    pub const TK0NF: u8 = 0x02;
    /// Address Mark Not Found
    pub const AMNF: u8 = 0x01;
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ATAIdentifyError {
    BusNotConnected = 0,
    NoDevice,
    DeviceIsNotATA,
    DeviceIsATAPI,
    DeviceIsSATA,
    Unknown = 255,
}

pub const PRIMARY_ATA_BUS: Mutex<ATABus> = Mutex::new(ATABus::new(0x1F0));
pub const SECONDARY_ATA_BUS: Mutex<ATABus> = Mutex::new(ATABus::new(0x170));
