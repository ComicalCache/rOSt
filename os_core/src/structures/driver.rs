use super::kernel_information::KernelInformation;

pub type Registrator = extern "C" fn(KernelInformation) -> Driver;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Driver {
    pub serial: [u8; 16],
}
