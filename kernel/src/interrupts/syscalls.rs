use x86_64::VirtAddr;

use crate::debug;

use super::gdt::GDT;

/// Sets up the LSTAR, FSTAR and STAR model-specific registers so it's possible to use `syscall`.
pub fn setup_syscalls() {
    use x86_64::registers::model_specific;
    use x86_64::registers::model_specific::{Efer, EferFlags};
    use x86_64::registers::rflags::RFlags;
    debug::log("Loading LSTAR, FSTAR and STAR");
    // LSTAR stores the address of the `syscall` handler.
    model_specific::LStar::write(VirtAddr::from_ptr(syscall_handler as *const ()));
    // FSTAR stores which bits of the flag register are cleared by `syscall`.
    model_specific::SFMask::write(RFlags::all());
    // STAR stores the indices of the GDT entries for the kernel and user descriptors.
    model_specific::Star::write(
        GDT.1.user_code_selector,
        GDT.1.user_data_selector,
        GDT.1.kernel_code_selector,
        GDT.1.kernel_data_selector,
    )
    .unwrap();
    let new_efer_flags = {
        let mut flags = Efer::read();
        flags.set(EferFlags::SYSTEM_CALL_EXTENSIONS, true);
        flags
    };
    unsafe {
        Efer::write(new_efer_flags);
    }
    debug::log("Syscalls active");
}

/// Handles a system call.
/// On entry to this function:
/// - the instruction pointer is stored in RCX
/// - the flags are stored in R11
/// - the stack pointer is still targeting the user mode stack
///
/// To properly handle this, we need to:
/// 1. save the user mode stack pointer
/// 2. set the syscall stack pointer
/// 3. save all the registers we need to preserve on the stack
/// 4. do our thing with the values we got from the user
/// 5. restore the registers from the stack
/// 6. restore the user mode stack pointer
/// 7. sysret (maybe setting the flags back?)
#[no_mangle]
unsafe extern "C" fn syscall_handler() {
    // TODO: handle syscall
    debug::log("Syscall");
    loop {}
}
