// This might be reimplemented from scratch in the future.

// TODO: implement all remaining interrupt handlers for CPU interrupts

mod cpu_handlers;
mod interrupt_register;
pub use interrupt_register::init_idt;
mod gdt;
pub use gdt::reload_gdt;
mod pic;
mod pic_handlers;

use crate::debug;

/// Initializes the PICs and enables interrupts
pub fn enable() {
    unsafe {
        // can cause undefined behaviour if the offsets were not set correctly
        pic::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
    debug::log("Interrupts enabled");
}
