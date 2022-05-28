use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{
    debug,
    interrupts::{
        cpu_handlers::{
            breakpoint_handler, double_fault_handler, general_protection_fault_handler,
            nmi_handler, page_fault_handler,
        },
        pic::InterruptIndex,
        pic_handlers::{
            _timer, ata_primary_interrupt_handler, ata_secondary_interrupt_handler,
            keyboard_interrupt_handler,
        },
    },
};
use x86_64::VirtAddr;

lazy_static! {
    /// The IDT used by the OS.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // ##################
        // # CPU interrupts #
        // ##################
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.non_maskable_interrupt
                .set_handler_fn(nmi_handler)
                .set_stack_index(crate::interrupts::gdt::NMI_IST_INDEX);

            idt.double_fault
                .set_handler_fn(double_fault_handler)
                // changes stack for double fault to avoid triple faults
                .set_stack_index(crate::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);

        // ##################
        // # PIC interrupts #
        // ##################
        unsafe {
            idt[InterruptIndex::Timer.as_usize()]
                .set_handler_addr(VirtAddr::from_ptr(_timer as *const ()))
                .set_stack_index(crate::interrupts::gdt::TIMER_IST_INDEX);
        }

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt[InterruptIndex::AtaPrimary.as_usize()]
            .set_handler_fn(ata_primary_interrupt_handler);

        idt[InterruptIndex::AtaSecondary.as_usize()]
            .set_handler_fn(ata_secondary_interrupt_handler);

        idt
    };
}

/// Loads the IDT.
pub fn init_idt() {
    IDT.load();
    debug::log("IDT loaded");
}
