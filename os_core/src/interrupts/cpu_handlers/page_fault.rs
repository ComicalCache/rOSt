use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::hlt_loop;
use crate::log_println;

/// Handles a page fault.
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    log_println!("EXCEPTION: PAGE FAULT");
    log_println!("Accessed Address: {:?}", Cr2::read());
    log_println!("Error Code: {:?}", error_code);
    log_println!("{:#?}", stack_frame);
    hlt_loop();
}
