#![no_std] // no standard library
#![no_main] // no entry point

// configures our test framework environment
#![feature(custom_test_frameworks)]
#![test_runner(os_testing::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::vga::text::interface::VGA_TEXT_BUFFER_INTERFACE;

use vga::text::color::Color;

mod offsets;
mod panic_handler;
mod test_framework;
mod vga;

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    // `cargo test` entry point.
    // ! DO NOT REMOVE!
    #[cfg(test)]
    test_main();

    // ########################################
    // # _start() actual entry on `cargo run` #
    // ########################################
    
    for i in 0..26 {
        println!("{i}");
    }

    VGA_TEXT_BUFFER_INTERFACE
        .lock()
        .set_color(Color::White, Color::DarkGray);
    VGA_TEXT_BUFFER_INTERFACE.lock().set_pos(10, 40);
    print!("Hello World{}", "!");
    VGA_TEXT_BUFFER_INTERFACE
        .lock()
        .set_color(Color::White, Color::Black);
    println!();
    VGA_TEXT_BUFFER_INTERFACE.lock().set_pos(40, 40); // this will panic

    loop {}
}
