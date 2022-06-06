use internal_utils::structures::kernel_information::KernelInformation;
use internal_utils::{serial_print, serial_println};

use crate::ansi_colors::Green;

/// Self documenting test runner trait
pub trait Testable {
    fn run(&self, kernel_information: KernelInformation);
}

impl<T> Testable for T
where
    T: Fn(KernelInformation),
{
    /// Runs the test and prints the result
    fn run(&self, kernel_information: KernelInformation) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self(kernel_information);
        serial_println!("{}", Green("[ok]"));
    }
}
