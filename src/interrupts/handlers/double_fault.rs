use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    // ! this should never do stack heavy operations because this handles has a separate stack
    // ! that has no stack guard page and thus could corrupt the stack
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
