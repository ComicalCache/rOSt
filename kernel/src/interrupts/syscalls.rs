use lazy_static::lazy_static;
use spin::Mutex;
use test_framework::serial_println;
use utils::syscall_name::SysCallName;
use x86_64::VirtAddr;

use crate::debug;

use super::gdt::GDT;
use core::arch::asm;

pub type SysCallHandlerFunc = extern "C" fn(u64, u64);

extern "C" fn fail_syscall(_arg1: u64, _arg2: u64) {
    panic!("NO SYSCALL DEFINED");
}

lazy_static! {
    static ref SYSCALLS: Mutex<[SysCallHandlerFunc; 1024]> = Mutex::new([fail_syscall; 1024]);
}

/// Sets up the LSTAR, FSTAR and STAR model-specific registers so it's possible to use `syscall`.
pub(crate) fn setup_syscalls() {
    use x86_64::registers::model_specific;
    use x86_64::registers::model_specific::{Efer, EferFlags};
    use x86_64::registers::rflags::RFlags;
    debug::log("Loading LSTAR, FSTAR and STAR");
    // LSTAR stores the address of the `syscall` handler.
    model_specific::LStar::write(VirtAddr::from_ptr(_syscall as *const ()));
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

#[allow(dead_code)]
pub fn register_syscall(syscall_number: u16, handler: SysCallHandlerFunc) {
    SYSCALLS.lock()[syscall_number as usize] = handler;
}

fn call_syscall(syscall_number: u16, arg1: u64, arg2: u64) {
    SYSCALLS.lock()[syscall_number as usize](arg1, arg2);
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
/// 7. sysretq (maybe setting the flags back?)
#[no_mangle]
#[naked]
unsafe extern "C" fn _syscall() -> ! {
    asm!(
        "mov r10, rsp",
        "mov rsp, 0x007F80014000", // User stack saved in R10, start of kernel stack loaded
        "push r10",
        "push rcx",
        "push r11",
        "push rbp",
        "push rbx", // save callee-saved registers
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        // We didn't touch RDI, RSI and RDX so we can just call the function with them.
        "call handler",
        // TODO: Returning using iret so we can return to kernel processes
        "pop r15", // restore callee-saved registers
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "pop r11",
        "pop rcx",
        "pop r10",
        "mov rsp, r10",
        "sysretq",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn handler(name: SysCallName, arg1: u64, arg2: u64) {
    // This block executes after saving the user state and before returning back
    serial_println!("syscall {:#?}", name);
    call_syscall(name as u64 as u16, arg1, arg2);
}
