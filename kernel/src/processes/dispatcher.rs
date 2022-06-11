use core::arch::asm;
use core::cell::RefCell;

use alloc::rc::Rc;
use x86_64::structures::paging::page::AddressNotAligned;
use x86_64::PhysAddr;

use crate::debug;
use crate::interrupts::GDT;
use crate::processes::Thread;
use internal_utils::get_current_tick;
use internal_utils::mov_all;

use super::get_scheduler;
use super::memory_mapper::clear_user_mode_mapping;
use super::RegistersState;

/// Runs the thread immediately.
pub fn switch_to_thread(thread: Rc<RefCell<Thread>>) -> ! {
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
        state = thread_mut.registers_state;
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
            "push r11", // rflags
            "push r13", // code selector
            "push r15", // instruction address to return to
            // Loading register state before jumping into thread
            mov_all!(),
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

/// Removes the thread from it's process. If this thread is the last one, the process is cleaned up.
pub fn exit_thread(thread: Rc<RefCell<Thread>>) -> Result<(), AddressNotAligned> {
    debug::log("Exiting thread");
    let borrowed_thread = thread.borrow();
    let mut borrowed_process = borrowed_thread.process.borrow_mut();
    let threads = { &mut borrowed_process.threads };
    threads.retain(|t| !Rc::ptr_eq(t, &thread));
    debug::log("Removed thread from process");
    {
        let scheduler = get_scheduler();
        if let Some(current_thread) = scheduler.running_thread.clone() {
            if Rc::ptr_eq(&thread, &current_thread) {
                scheduler.running_thread = None;
            }
        }
    }
    if threads.is_empty() {
        //Clean up the process
        get_scheduler().remove_process(borrowed_thread.process.clone());
        debug::log("Removed process from scheduler");
        unsafe {
            clear_user_mode_mapping(borrowed_process.cr3)?;
        }
    }
    Ok(())
}
