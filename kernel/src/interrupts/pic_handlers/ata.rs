use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupts::pic::{InterruptIndex, PICS};

pub extern "x86-interrupt" fn ata_primary_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::AtaPrimary.as_u8());
    }
}

pub extern "x86-interrupt" fn ata_secondary_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::AtaSecondary.as_u8());
    }
}
