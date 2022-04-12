#![no_std] // no standard library
#![no_main] // no entry point

use crate::vga_buffer::VGA_TEXT_BUFFER_WRITER;

use vga_buffer::Color;

mod offsets;
mod panic_handler;
mod vga_buffer;

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    for i in 0..26 {
        println!("{i}");
    }

    VGA_TEXT_BUFFER_WRITER.lock().set_color(Color::White, Color::DarkGray);
    VGA_TEXT_BUFFER_WRITER.lock().set_pos(10, 40);
    print!("Hello World{}", "!");
    VGA_TEXT_BUFFER_WRITER.lock().set_color(Color::White, Color::Black);
    println!();
    VGA_TEXT_BUFFER_WRITER.lock().set_pos(40, 40); // this will panic

    loop {}
}
