use core::cell::RefCell;

use alloc::{collections::VecDeque, rc::Rc};

use super::{process::Process, thread::Thread, RegistersState};
use crate::processes::dispatcher::switch_to_thread;

static mut SCHEDULER: Option<Scheduler> = None;

pub fn get_scheduler() -> &'static mut Scheduler {
    unsafe { SCHEDULER.get_or_insert_default() }
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

pub fn run_next_thread() -> Option<()> {
    let next_thread = get_scheduler().schedule();
    if let Some(thread) = next_thread {
        crate::processes::dispatcher::switch_to_thread(thread);
    } else {
        Some(())
    }
}

#[derive(Default)]
pub struct Scheduler {
    /// The currently running process.
    pub running_thread: Option<Rc<RefCell<Thread>>>,
    /// The list of processes that are registered.
    processes: VecDeque<Rc<RefCell<Process>>>,
}

impl Scheduler {
    /// Adds a process to the scheduling queue so it will be ran.
    pub fn add_process(&mut self, process: Process) -> Rc<RefCell<Process>> {
        let rc = Rc::new(RefCell::new(process));
        self.processes.push_back(rc.clone());
        rc
    }

    /// Removes the process from the queue.
    pub fn remove_process(&mut self, process: Rc<RefCell<Process>>) {
        self.processes.retain(|p| !Rc::ptr_eq(p, &process));
    }

    /// Manages scheduler operations on a timer tick
    pub fn timer_tick(&self, registers_state: RegistersState, tick: u64) {
        if let Some(thread) = self.running_thread.clone() {
            let mut thread_mut = thread.borrow_mut();

            thread_mut.registers_state = registers_state;
            thread_mut.total_ticks += tick - thread_mut.last_tick;
            thread_mut.last_tick = tick;
            let mut process = thread_mut.process.borrow_mut();
            process.total_ticks += tick - process.last_tick;
            process.last_tick = tick;
        }
    }

    /// Returns the thread that should be ran next.
    pub fn schedule(&mut self) -> Option<Rc<RefCell<Thread>>> {
        if self.processes.is_empty() {
            return None;
        }
        // We're taking the first process in the queue that returns a runnable thread
        let processes = &self.processes;
        let process_index = processes.iter().position(|process| {
            Process::update_sleeping_threads(process.clone());
            !process.borrow().ready_threads.is_empty()
        })?;
        let process = self.processes.remove(process_index)?;
        let thread = Scheduler::get_thread_to_run(process.clone())?;
        // Putting the process at the back of the queue
        self.processes.push_back(process);

        Some(thread)
    }

    /// Returns the thread from the process that should be ran next.
    fn get_thread_to_run(process: Rc<RefCell<Process>>) -> Option<Rc<RefCell<Thread>>> {
        let mut process_borrowed = process.borrow_mut();
        // Taking the first thread in the chosen process
        if process_borrowed.ready_threads.is_empty() {
            return None;
        }
        let thread = process_borrowed.ready_threads.remove(0);
        // Putting the thread at the back of the thread-queue
        process_borrowed.ready_threads.push(thread.clone());
        Some(thread)
    }
}
