use kernel::processes::dispatcher::exit_thread;
use kernel::processes::{get_scheduler, run_next_thread};

use crate::syscall_name::SysCallName;

pub(crate) extern "C" fn thread_exit(code: u64, _: u64) -> u64 {
    let scheduler = get_scheduler();
    if let Some(thread) = scheduler.running_thread.clone() {
        exit_thread(thread).unwrap();
        run_next_thread();
        panic!("No threads to run");
    }
    code
}

pub extern "C" fn exit(status: u64) -> ! {
    crate::syscall(SysCallName::ThreadExit, status, 0);
    panic!("Thread exited");
}
