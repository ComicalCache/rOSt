use crate::log_println;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
