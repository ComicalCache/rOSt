use core::panic::PanicInfo;

use crate::{print, vga::text::writer::VGA_TEXT_BUFFER_WRITER};

#[panic_handler]
// this function is called if a panic occurs
fn panic(info: &PanicInfo) -> ! {
    // needs to force unlock the VgaTextBufferWriter because it could be locked when the panic occurs
    // this should be safe because the panic handler is the last thing that is executed 
    unsafe { VGA_TEXT_BUFFER_WRITER.force_unlock() };
    VGA_TEXT_BUFFER_WRITER.lock().__set_panic_config();
    
    print!("{}", info);

    loop {}
}
