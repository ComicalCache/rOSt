use core::cell::RefCell;

use alloc::{rc::Rc, sync::Arc, vec::Vec};

use super::{Process, Thread};
use crate::processes::dispatcher::switch_to_thread;

static mut SCHEDULER: Scheduler = Scheduler::new();

pub(crate) fn get_scheduler() -> &'static mut Scheduler {
    unsafe { &mut SCHEDULER }
}

/// Runs the scheduler, giving it control of the CPU.
///
/// Will return only if there are no threads at all to run.
pub fn run_processes() -> Option<()> {
    switch_to_thread(get_scheduler().schedule()?);
}

pub fn add_process(process: Process) -> Rc<RefCell<Process>> {
    get_scheduler().add_process(process)
}

pub(crate) struct Scheduler {
    /// The currently running process.
    pub running_thread: Option<Rc<RefCell<Thread>>>,
    /// The list of processes that are registered.
    processes: Vec<Rc<RefCell<Process>>>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            running_thread: None,
            processes: Vec::new(),
        }
    }

    /// Adds a process to the scheduling queue so it will be ran.
    pub fn add_process(&mut self, process: Process) -> Rc<RefCell<Process>> {
        let rc = Rc::new(RefCell::new(process));
        self.processes.push(rc.clone());
        rc
    }

    /// Returns the thread that should be ran next.
    pub fn schedule(&mut self) -> Option<Rc<RefCell<Thread>>> {
        if self.processes.is_empty() {
            return None;
        }
        // We're taking the first process in the queue
        let process = self.processes.remove(0);
        let thread = {
            let process_cloned = process.clone();
            let mut process_borrowed = process_cloned.borrow_mut();
            // Taking the first thread in the chosen process
            let thread = process_borrowed.threads.remove(0);
            // Putting the thread at the back of the thread-queue
            process_borrowed.threads.push(thread.clone());
            thread
        };
        // Putting the process at the back of the queue
        self.processes.push(process);

        Some(thread)
    }
}
