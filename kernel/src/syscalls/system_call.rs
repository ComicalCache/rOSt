use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::VirtAddr;

use super::syscall_name::SysCallName;
use crate::{debug, memory::with_kernel_memory, processes::get_scheduler};

use crate::interrupts::gdt::GDT;
use core::arch::asm;
use utils::{mov_all, push_all};

pub type SysCallHandlerFunc = extern "C" fn(u64, u64) -> u64;

/// A system call handler that panics.
extern "C" fn fail_syscall(_arg1: u64, _arg2: u64) -> u64 {
    panic!("NO SYSCALL DEFINED");
}

lazy_static! {
    static ref SYSCALLS: Mutex<[SysCallHandlerFunc; 1024]> = Mutex::new([fail_syscall; 1024]);
}

/// Sets up the LSTAR, FSTAR and STAR model-specific registers so it's possible to use `syscall`.
///
/// [OSDev - System calls](https://wiki.osdev.org/System_Calls#Sysenter.2FSysexit_.28Intel.29)
///
/// [OSDev - Syscall instruction](https://wiki.osdev.org/Sysenter#AMD:_SYSCALL.2FSYSRET)
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
/// Registers a system call with a handler.
pub fn register_syscall(syscall_number: u16, handler: SysCallHandlerFunc) {
    SYSCALLS.lock()[syscall_number as usize] = handler;
}

/// Handles a system call.
/// On entry to this function:
/// - the instruction pointer is stored in RCX
/// - the flags are stored in R11
/// - the stack pointer is still targeting the user mode stack
///
/// [OSDev - System calls](https://wiki.osdev.org/System_Calls#Sysenter.2FSysexit_.28Intel.29)
///
/// [OSDev - Syscall instruction](https://wiki.osdev.org/Sysenter#AMD:_SYSCALL.2FSYSRET)
///
/// [Intel manual - IRETQ stack frame structure - points 6.14.3 and 6.14.4](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-system-programming-manual-325384.pdf)
#[no_mangle]
#[naked]
unsafe extern "C" fn _syscall() -> ! {
    asm!(
        // We disable the interrupts for the duration of the syscall.
        "cli",
        // We save the user stack in R10.
        "mov r10, rsp",
        // We load the start of kernel stack.
        "mov rsp, 0x007F80014000",
        // We save the state of the thread to the stack.
        push_all!(),
        // We save the pointer to the saved thread state (RSP points to the push_all structure).
        "mov r9, rsp",
        // Then we save that pointer on the stack so we don't lose it when we call functions.
        "push r9",
        // We call the C abi handler
        "  call handler",
        // Return value is in RAX so we save that on the stack
        "  push rax",
        "    call get_code_selector",
        "    push rax",
        "      call get_data_selector",
        // RAX has the data selector, we pop the code selector to RBX
        "    pop rbx",
        "    mov rcx, rax", // Now the selectors are in RBX & RCX
        // We get the syscall value (return value from handler) back
        "  pop rax",
        // We get the thread state pointer back
        "pop r9",
        // We need to use iretq instead of sysret because we want to support Ring 0 processes.
        // Unfortunately sysret always returns to Ring 3, so we have to use a more complicated method.
        // Preparing iretq - we have to push a few things on the stack for iretq to work.
        // Data selector
        "push rcx",
        // Process stack pointer
        "push [r9 + 40]",
        // Flags - we prepare them so interrupts will be turned on when we return
        "mov r11, [r9 + 32]",
        "or r11, 0x200",
        "and r11, 0xffffffffffffbfff",
        // Flags
        "push r11",
        // Code selector
        "push rbx",
        // Instruction address to return to
        "push [r9 + 96]",
        // We want to keep the RAX value, so we save it on the stack
        "push rax",
        // And we load all the registers from the thread state
        mov_all!(),
        // And we pop RAX so the thread state is the same except RAX has the return value
        "pop rax",
        // We return from the syscall
        "iretq",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn handler(name: SysCallName, arg1: u64, arg2: u64) -> u64 {
    // This block executes after saving the user state and before returning back
    with_kernel_memory(|| SYSCALLS.lock()[name as u64 as u16 as usize](arg1, arg2))
}

// TODO Try to combine both of these functions to make it faster.
// We should be able to return a C-style struct with two u64s and manage it through ASM.
#[no_mangle]
extern "C" fn get_code_selector() -> u64 {
    with_kernel_memory(|| {
        let thread = get_scheduler().running_thread.clone().unwrap();
        let thread = thread.as_ref().borrow();
        let process = thread.process.as_ref().borrow();
        if process.kernel_process {
            (GDT.1.kernel_code_selector.index() * 8) as u64
        } else {
            ((GDT.1.user_code_selector.index() * 8) | 3) as u64
        }
    })
}

// TODO Try to combine both of these functions to make it faster.
// We should be able to return a C-style struct with two u64s and manage it through ASM.
#[no_mangle]
extern "C" fn get_data_selector() -> u64 {
    with_kernel_memory(|| {
        let thread = get_scheduler().running_thread.clone().unwrap();
        let thread = thread.as_ref().borrow();
        let process = thread.process.as_ref().borrow();
        if process.kernel_process {
            (GDT.1.kernel_data_selector.index() * 8) as u64
        } else {
            ((GDT.1.user_data_selector.index() * 8) | 3) as u64
        }
    })
}
