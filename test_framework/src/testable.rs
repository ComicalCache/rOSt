use crate::ansi_colors::Green;
use crate::{serial_print, serial_println};

/// Self documenting test runner trait
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    /// Runs the test and prints the result
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("{}", Green("[ok]"));
    }
}
