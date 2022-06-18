use crate::interrupts::pic::{InterruptIndex, PICS};
use crate::memory::with_kernel_memory;
use crate::processes::{get_scheduler, run_next_thread, RegistersState};
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
        get_scheduler().timer_tick(registers_state, tick);
        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
        }
        run_next_thread();
    });
}
