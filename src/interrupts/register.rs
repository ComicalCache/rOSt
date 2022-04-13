use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::handlers::{breakpoint_handler, double_fault_handler};

lazy_static! {
    /// The IDT used by the OS.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // sets all the interrupt handlers for the appropriate interrupts
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                // changes stack for double fault to avoid triple faults
                .set_stack_index(crate::interrupts::gtd::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
