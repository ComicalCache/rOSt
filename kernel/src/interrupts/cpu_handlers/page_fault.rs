use utils::serial_println;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::hlt_loop;

/// Handles a page fault.
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    x86_64::instructions::interrupts::disable();

    serial_println!("EXCEPTION: PAGE FAULT");
    serial_println!("{:?}", error_code);
    serial_println!("Page: {:X?}", Cr2::read_raw());
    serial_println!("{:#?}", stack_frame);
    hlt_loop();
}
