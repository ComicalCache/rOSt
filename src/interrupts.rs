// This might be reimplemented from scratch in the future.

// TODO: implement all remaining interrupt handlers for CPU interrupts

mod cpu_handlers;
pub mod interrupt_register;
mod pic_handlers;
pub use interrupt_register::init_idt;
pub mod gdt;
pub use gdt::init_gdt;
pub mod pic;
pub use pic::PICS;
