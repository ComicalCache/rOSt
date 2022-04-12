#![no_std] // no standard library
#![no_main] // no entry point

// configures our test framework environment
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_framework::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::vga::text::writer::VGA_TEXT_BUFFER_WRITER;

use vga::text::color::Color;

mod offsets;
mod panic_handler;
mod test_framework;
mod tests;
mod vga;

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    // conditional compilation, only included on `cargo test`, else discarded.
    // Avoids compiler warnings about unused code
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
