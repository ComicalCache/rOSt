use core::panic::PanicInfo;

#[cfg(not(test))]
use crate::{print, vga::text::interface::VGA_TEXT_BUFFER_WRITER};

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

#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    os_testing::test_panic_handler(info)
}
