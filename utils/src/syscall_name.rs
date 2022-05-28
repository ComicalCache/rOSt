#[repr(u64)]
#[derive(Debug)]
pub enum SysCallName {
    Debug = 0,
    SecondDebug = 1,
}
