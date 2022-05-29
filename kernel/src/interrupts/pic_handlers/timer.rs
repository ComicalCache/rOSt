use crate::interrupts::pic::{InterruptIndex, PICS};
use crate::memory::switch_to_kernel_memory;
use crate::processes::{get_scheduler, RegistersState};
use core::arch::asm;
use utils::get_current_tick;
use utils::{pop_all, push_all};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PhysFrame;
use x86_64::PhysAddr;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _timer() -> ! {
    asm!(
        // We have RFLAGS and RIP on the stack already.
        push_all!(),
        "mov rdi, rsp",
        "mov rsi, cr3",
        "call timer_interrupt_handler",
        pop_all!(),
        "iretq",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn timer_interrupt_handler(registers_state: *const RegistersState, cr3: PhysAddr) {
    let registers_state = unsafe { *registers_state };
    let tick = get_current_tick();
    switch_to_kernel_memory();
    {
        if let Some(thread) = get_scheduler().running_thread.clone() {
            {
                let mut thread_mut = thread.borrow_mut();

                thread_mut.registers_state = registers_state;
                thread_mut.total_ticks += tick - thread_mut.last_tick;
                thread_mut.last_tick = tick;
                let mut process = thread_mut.process.borrow_mut();
                process.total_ticks += tick - process.last_tick;
                process.last_tick = tick;
            }
            get_scheduler().add_thread(thread);
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
    let next_thread = get_scheduler().schedule();
    if let Some(thread) = next_thread {
        crate::processes::dispatcher::run_thread(thread);
    }
    unsafe {
        Cr3::write(
            PhysFrame::from_start_address_unchecked(cr3),
            Cr3Flags::empty(),
        );
    }
}
