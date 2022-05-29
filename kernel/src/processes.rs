pub mod dispatcher;

mod memory_mapper;

mod process;
pub use process::Process;

mod thread;
pub use thread::Thread;

mod registers_state;
pub use registers_state::RegistersState;

mod scheduler;
pub(crate) use scheduler::get_scheduler;
pub use scheduler::{add_process, run_processes};

pub type ProcessFunction = extern "C" fn();
