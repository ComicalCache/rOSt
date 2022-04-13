// This might be reimplemented from scratch in the future.

mod handlers;
pub mod register; pub use register::init_idt;
pub mod gtd; pub use gtd::init_gdt;
