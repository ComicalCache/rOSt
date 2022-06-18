use core::cell::RefCell;

use alloc::rc::Rc;
use kernel::processes::dispatcher::exit_thread;
use kernel::processes::run_next_thread;
use kernel::processes::thread::Thread;

use crate::syscall_name::SysCallName;

pub(crate) extern "C" fn thread_exit(code: u64, _: u64, caller: Rc<RefCell<Thread>>) -> u64 {
    exit_thread(caller).unwrap();
    run_next_thread();
    panic!("No threads to run");
}

pub extern "C" fn exit(status: u64) -> ! {
    crate::syscall(SysCallName::ThreadExit, status, 0);
    panic!("Thread exited");
}
