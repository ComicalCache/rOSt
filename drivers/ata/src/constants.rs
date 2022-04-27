use alloc::sync::Arc;
use spin::Mutex;

use super::bus::ATABus;
use bitflags::bitflags;
use lazy_static::lazy_static;

bitflags! {
    #[repr(C)]
    pub struct StatusRegisterFlags: u8 {
        /// Busy
        const BSY = 0b10000000;
        /// Device Ready
        const DRDY = 0b01000000;
        /// Device Fault
        const DF = 0b00100000;
        /// Seek Complete
        const DSC = 0b00010000;
        /// Data Transfer Required
        const DRQ = 0b00001000;
        /// Data Corrected
        const CORR = 0b00000100;
        /// Index Mark
        const IDX = 0b00000010;
        /// Error
        const ERR = 0b00000001;
    }

    #[repr(C)]
    pub struct ErrorRegisterFlags: u8 {
    /// Bad Block
        const BBK = 0b10000000;
    /// Uncorrectable Data Error
        const UNC = 0b10000000;
    /// Media Changed
        const MC = 0b10000000;
    /// ID Mark Not Found
        const IDNF = 0b10000000;
    /// Media Change Requested
        const MCR = 0b10000000;
    /// Command Aborted
        const ABRT = 0b10000000;
    /// Track 0 Not Found
        const TK0NF = 0b10000000;
    /// Address Mark Not Found
        const AMNF = 0b10000000;
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
