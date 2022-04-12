use core::panic::PanicInfo;

// conditional compilation, only included on `cargo run`, else discarded. 
// Avoids compiler warnings about unused code
#[cfg(not(test))]
use crate::{print, vga::text::writer::VGA_TEXT_BUFFER_WRITER};

// conditional compilation, only included on `cargo run`, else discarded. 
// Avoids compiler warnings about unused code
#[cfg(not(test))]
#[panic_handler]
// this function is called if a panic occurs and is not a test, all output is redirected to the VGA buffer
fn panic(info: &PanicInfo) -> ! {
    // needs to force unlock the VgaTextBufferWriter because it could be locked when the panic occurs
    // this should be safe because the panic handler is the last thing that is executed
    unsafe { VGA_TEXT_BUFFER_WRITER.force_unlock() };
    VGA_TEXT_BUFFER_WRITER.lock().__set_panic_config();

    print!("{}", info);

    loop {}
}

// conditional compilation, only included on `cargo test`, else discarded. 
// Avoids compiler warnings about unused code
#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    use crate::{
        serial_println,
        test_framework::qemu_exit::{exit_qemu, QemuExitCode},
    };

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}
