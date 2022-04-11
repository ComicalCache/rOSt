#![no_std] // no standard library
#![no_main] // no entry point

mod panic_handler;
mod offsets;

// test hello world function
fn print_hello_world() {
    static HELLO_WORLD: &[u8] = b"Hello, world!";

    let vga_buffer = offsets::VGA_BUFFER as *mut u8;

    for (i, &byte) in HELLO_WORLD.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[no_mangle]
// entry point of the program
pub extern "C" fn _start() -> ! {
    print_hello_world();

    loop {}
}
