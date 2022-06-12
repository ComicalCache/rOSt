pub mod dispatcher;

mod memory_mapper;

mod process;
pub use process::Process;

mod thread;
pub use thread::Thread;

mod registers_state;
pub use registers_state::RegistersState;

mod scheduler;
pub use scheduler::{add_process, get_scheduler, run_next_thread, run_processes};

pub type ProcessFunction = extern "C" fn();
