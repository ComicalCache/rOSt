// This might be reimplemented from scratch in the future.

// TODO: implement all remaining interrupt handlers for CPU interrupts

mod cpu_handlers;
mod interrupt_register;
pub use interrupt_register::init_idt;
mod gdt;
mod pic_handlers;
pub use gdt::{reload_gdt, run_in_user_mode};
mod pic;
mod syscalls;
pub use syscalls::setup_syscalls;

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
