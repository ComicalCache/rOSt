use alloc::sync::Arc;
use spin::Mutex;

use super::bus::ATABus;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StatusRegisterFlags {
    /// Busy
    BSY = 0x80,
    /// Device Ready
    DRDY = 0x40,
    /// Device Fault
    DF = 0x20,
    /// Seek Complete
    DSC = 0x10,
    /// Data Transfer Required
    DRQ = 0x08,
    /// Data Corrected
    CORR = 0x04,
    /// Index Mark
    IDX = 0x02,
    /// Error
    ERR = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorRegisterFlags {
    /// Bad Block
    BBK = 0x80,
    /// Uncorrectable Data Error
    UNC = 0x40,
    /// Media Changed
    MC = 0x20,
    /// ID Mark Not Found
    IDNF = 0x10,
    /// Media Change Requested
    MCR = 0x08,
    /// Command Aborted
    ABRT = 0x04,
    /// Track 0 Not Found
    TK0NF = 0x02,
    /// Address Mark Not Found
    AMNF = 0x01,
}

pub(crate) fn get_error_flag(error: u8) -> ErrorRegisterFlags {
    match error {
        0x80 => ErrorRegisterFlags::BBK,
        0x40 => ErrorRegisterFlags::UNC,
        0x20 => ErrorRegisterFlags::MC,
        0x10 => ErrorRegisterFlags::IDNF,
        0x08 => ErrorRegisterFlags::MCR,
        0x04 => ErrorRegisterFlags::ABRT,
        0x02 => ErrorRegisterFlags::TK0NF,
        0x01 => ErrorRegisterFlags::AMNF,
        _ => ErrorRegisterFlags::UNC,
    }
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
#[non_exhaustive]
pub enum ATACommands {
    Identify = 0xEC,
    WriteSectors = 0x30,
    ReadSectors = 0x20,
    CacheFlush = 0xE7,
}

lazy_static! {
    pub static ref PRIMARY_ATA_BUS: Arc<Mutex<ATABus>> = Arc::new(Mutex::new(ATABus::new(0x1F0)));
    pub static ref SECONDARY_ATA_BUS: Arc<Mutex<ATABus>> = Arc::new(Mutex::new(ATABus::new(0x170)));
}
pub const PARTITION_ID: u8 = 0xED;
