use crate::{serial_print, serial_println};

#[test_case]
pub fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}