#[repr(u64)]
#[derive(Debug)]
pub enum SysCallName {
    FirstProcess = 0,
    SecondProcess = 1,
}
