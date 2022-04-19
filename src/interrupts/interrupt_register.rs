use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::{
    cpu_handlers::{breakpoint_handler, double_fault_handler, page_fault_handler},
    pic::InterruptIndex,
    pic_handlers::{keyboard_interrupt_handler, timer_interrupt_handler},
};

lazy_static! {
    /// The IDT used by the OS.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // sets all the interrupt handlers for the appropriate CPU interrupts
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                // changes stack for double fault to avoid triple faults
                .set_stack_index(crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);

        // sets all the interrupt handlers for the appropriate PIC interrupts
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
