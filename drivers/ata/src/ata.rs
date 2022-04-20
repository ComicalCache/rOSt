use os_core::structures::kernel_information::KernelInformation;

pub mod bus;
pub mod constants;
pub mod descriptor;

pub extern "C" fn driver_init(kernel_info: KernelInformation) {}
