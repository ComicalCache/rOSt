use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::handlers::breakpoint_handler;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}