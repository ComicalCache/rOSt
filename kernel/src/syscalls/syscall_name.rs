#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysCallName {
    FirstProcess = 0,
    SecondProcess = 1,
    ThreadExit = 300
}
