use x86_64::structures::idt::InterruptStackFrame;

/// Handles a non-maskable interrupt.
///
pub extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame) {
    // ! this should never do stack heavy operations because this handles has a separate stack
    // ! that has no stack guard page and thus could corrupt the stack
    panic!("EXCEPTION: NMI\n{:#?}", stack_frame);
}
