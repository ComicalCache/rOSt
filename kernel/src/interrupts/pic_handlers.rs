mod timer;
pub use timer::_timer;
mod keyboard;
pub use keyboard::keyboard_interrupt_handler;
mod ata;
pub use ata::{ata_primary_interrupt_handler, ata_secondary_interrupt_handler};
mod addresses;
