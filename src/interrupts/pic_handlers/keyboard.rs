use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1, DecodedKey};
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use lazy_static::lazy_static;

use crate::{
    interrupts::{
        pic::InterruptIndex, pic_handlers::addresses::PS2_INTERRUPT_CONTROLLER_SCAN_CODE_PORT, PICS,
    },
    print,
};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(PS2_INTERRUPT_CONTROLLER_SCAN_CODE_PORT);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                // ! this introduces deadlock potential because print will lock the VgaTextBufferInterface
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
