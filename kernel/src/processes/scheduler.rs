use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

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

pub(crate) fn add_thread(thread: Rc<RefCell<Thread>>) {
    get_scheduler().add_thread(thread);
}

pub fn add_process(process: Process) -> Rc<RefCell<Process>> {
    get_scheduler().add_process(process)
}

pub(crate) struct Scheduler {
    /// The currently running process.
    pub running_thread: Option<Rc<RefCell<Thread>>>,
    /// The list of threads that are ready to run.
    threads: Vec<Rc<RefCell<Thread>>>,
    /// The list of processes that are registered.
    processes: Vec<Rc<RefCell<Process>>>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            running_thread: None,
            threads: Vec::new(),
            processes: Vec::new(),
        }
    }

    /// Adds a process to the scheduling queue so it will be ran.
    pub fn add_process(&mut self, process: Process) -> Rc<RefCell<Process>> {
        process
            .threads
            .iter()
            .for_each(|thread| self.add_thread(thread.clone()));
        let rc = Rc::new(RefCell::new(process));
        self.processes.push(rc.clone());
        rc
    }

    /// Adds the thread to the queue so it can be ran later.
    pub fn add_thread(&mut self, thread: Rc<RefCell<Thread>>) {
        self.threads.push(thread);
    }

    /// Returns the thread that should be ran next.
    ///
    /// This action removes the thread from the waiting queue - be sure to add it back using `return_thread` if it should be ran again.
    pub fn schedule(&mut self) -> Option<Rc<RefCell<Thread>>> {
        if self.threads.is_empty() {
            return None;
        }
        Some(self.threads.remove(0))
    }
}
