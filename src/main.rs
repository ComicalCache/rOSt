#![no_std] // no standard library
#![no_main] // no entry point

#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::vga::text::writer::VGA_TEXT_BUFFER_WRITER;

use vga::text::color::Color;

mod offsets;
mod panic_handler;
mod tests;
mod vga;

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    // only ran on `cargo test`
    #[cfg(test)]
    test_main();
    
    // ! _start() actual entry on `cargo run`
    for i in 0..26 {
        println!("{i}");
    }

    VGA_TEXT_BUFFER_WRITER
        .lock()
        .set_color(Color::White, Color::DarkGray);
    VGA_TEXT_BUFFER_WRITER.lock().set_pos(10, 40);
    print!("Hello World{}", "!");
    VGA_TEXT_BUFFER_WRITER
        .lock()
        .set_color(Color::White, Color::Black);
    println!();
    VGA_TEXT_BUFFER_WRITER.lock().set_pos(40, 40); // this will panic

    loop {}
}
