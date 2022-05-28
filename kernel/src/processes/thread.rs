use core::cell::RefCell;

use alloc::rc::Rc;
use utils::get_current_tick;
use x86_64::VirtAddr;

use super::Process;

use super::scheduler::add_thread;
use super::RegistersState;

#[derive(Debug)]
pub struct Thread {
    /// The thread's ID.
    pub id: u64,
    /// The state of the registers.
    pub registers_state: RegistersState,
    /// Total ticks the process has been running for.
    pub total_ticks: u64,
    /// The tick the process has been created on.
    pub start_tick: u64,
    /// The tick the process has been last ran on.
    pub last_tick: u64,
    /// The process the thread is running for.
    pub process: Rc<RefCell<Process>>,
}

impl Thread {
    /// Returns the percentage of ticks the thread spent running, calculated from the creation time of the thread
    pub fn tick_density(&self, current_tick: u64) -> u64 {
        let ticks_maximum = current_tick - self.start_tick;
        self.total_ticks * 100 / ticks_maximum
    }

    /// Creates a new thread with the given starting address and stack pointer.
    pub fn new(
        address: u64,
        stack_pointer: u64,
        process: Rc<RefCell<Process>>,
    ) -> Rc<RefCell<Self>> {
        let thread = Thread {
            id: process.borrow().threads.len() as u64,
            total_ticks: 0,
            start_tick: get_current_tick(),
            last_tick: 0,
            process: process.clone(),
            registers_state: RegistersState::new(
                VirtAddr::new(address),
                0x200,
                VirtAddr::new(stack_pointer),
            ),
        };
        let rc = Rc::new(RefCell::new(thread));
        process.borrow_mut().threads.push(rc.clone());
        add_thread(rc.clone());
        rc
    }
}
