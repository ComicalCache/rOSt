mod syscall_name;
mod system_call;
pub use syscall_name::SysCallName;
pub use system_call::register_syscall;
pub(crate) use system_call::setup_syscalls;
