use super::kernel_information::KernelInformation;

/// extern C for future compatibility with loading drivers from the file system
pub type Registrator = extern "C" fn(KernelInformation) -> Driver;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Driver {
    /// The signature of the driver. Should be unique through all the drivers.
    pub signature: [u8; 16],
}
