use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Stores the interrupt address for a given interrupt type
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// Stores the interrupt address for a given interrupt type
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,

    AtaPrimary = PIC_1_OFFSET + 14,
    AtaSecondary,
}

impl InterruptIndex {
    /// Returns the corresponding interrupt number for this interrupt type as a u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the corresponding interrupt number for this interrupt type as a usize
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

/// The PICs of the system.
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    // this is unsafe, because wrong offsets will cause undefined behavior
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});
