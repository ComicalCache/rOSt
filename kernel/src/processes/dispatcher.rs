use core::arch::asm;
use core::cell::RefCell;

use alloc::rc::Rc;
use x86_64::PhysAddr;

use crate::interrupts::GDT;
use crate::processes::Thread;
use utils::get_current_tick;

use super::get_scheduler;
use super::RegistersState;

/// Runs the thread immediately.
pub fn run_thread(thread: Rc<RefCell<Thread>>) -> ! {
    let code_selector_id: u64;
    let data_selector_id: u64;
    let cr3: PhysAddr;
    let state: RegistersState;
    x86_64::instructions::interrupts::disable();
    {
        let tick = get_current_tick();
        let mut thread_mut = thread.borrow_mut();
        thread_mut.last_tick = tick;
        let mut process = thread_mut.process.borrow_mut();
        process.last_tick = tick;
        code_selector_id = if process.kernel_process {
            (GDT.1.kernel_code_selector.index() * 8) as u64
        } else {
            ((GDT.1.user_code_selector.index() * 8) | 3) as u64
        };
        data_selector_id = if process.kernel_process {
            (GDT.1.kernel_data_selector.index() * 8) as u64
        } else {
            ((GDT.1.user_data_selector.index() * 8) | 3) as u64
        };
        cr3 = process.cr3;
        state = thread_mut.registers_state.clone();
    }

    get_scheduler().running_thread.replace(thread.clone());
    unsafe {
        // We decrement the counter forcefully because that function doesn't return by Rust.
        Rc::decrement_strong_count(Rc::into_raw(thread));
        asm!(
            "mov cr3, r10",
            "push r14", // data selector
            "push r12", // process stack pointer
            "or r11, 0x200",
            "and r11, 0xffffffffffffbfff",
            "push r11", // eflags
            "push r13", // code selector
            "push r15", // instruction address to return to
            // Loading register state before jumping into thread
            "mov r15, [r9]",
            "mov r14, [r9 + 8]",
            "mov r13, [r9 + 16]",
            "mov r12, [r9 + 24]",
            "mov r11, [r9 + 32]",
            "mov r10, [r9 + 40]",
            "mov r8, [r9 + 56]",
            "mov rdi, [r9 + 64]",
            "mov rsi, [r9 + 72]",
            "mov rbp, [r9 + 80]",
            "mov rdx, [r9 + 88]",
            "mov rcx, [r9 + 96]",
            "mov rbx, [r9 + 104]",
            "mov rax, [r9 + 112]",
            "mov r9, [r9 + 48]",
            "iretq",
            in("r9") (&state as *const RegistersState as *const u8),
            in("r10") (cr3.as_u64()),
            in("r11") (state.rflags),
            in("r12") (state.rsp.as_u64()),
            in("r13") (code_selector_id),
            in("r14") (data_selector_id),
            in("r15") (state.rip.as_u64()),
            options(noreturn)
        );
    }
}
