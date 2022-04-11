use core::panic::PanicInfo;

#[panic_handler]
// this function is called if a panic occurs
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
