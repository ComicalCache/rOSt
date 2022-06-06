use crate::interrupts::pic::{InterruptIndex, PICS};
use crate::memory::with_kernel_memory;
use crate::processes::{get_scheduler, RegistersState};
use core::arch::asm;
use internal_utils::get_current_tick;
use internal_utils::{pop_all, push_all};

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _timer() -> ! {
    asm!(
        // We have RFLAGS and RIP on the stack already.
        push_all!(),
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        pop_all!(),
        "iretq",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn timer_interrupt_handler(registers_state: *const RegistersState) {
    let registers_state = unsafe { *registers_state };
    let tick = get_current_tick();

    with_kernel_memory(|| {
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
    });
}
