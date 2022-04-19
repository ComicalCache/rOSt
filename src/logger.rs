use core::fmt;

use crate::vga::{
    vga_buffer::{VGADevice, VGADeviceFactory},
    vga_color,
    vga_core::{Clearable, TextDrawable, CHAR_HEIGHT},
};
use bootloader::boot_info::FrameBuffer;
use lazy_static::lazy_static;
use spin::Mutex;

const LOGGER_START_INDENT_X: u16 = 32;
const LOGGER_START_INDENT_Y: u16 = 2 * CHAR_HEIGHT as u16;

lazy_static! {
    /// Global logger instance that prints to the BSOD screen.
    pub static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(Option::None);
}

/// Initialises the logger.
pub fn init(framebuffer: &'static mut FrameBuffer) {
    let _ = LOGGER.lock().insert(Logger {
        x: 0,
        start_x: 0,
        y: 0,
        device: VGADeviceFactory::from_buffer(framebuffer),
        took_over: false,
    });
}

/// Logger struct that prints to the BSOD screen.
pub struct Logger {
    x: u16,
    y: u16,
    start_x: u16,
    device: VGADevice<'static>,
    took_over: bool,
}

impl Logger {
    fn __log(&mut self, text: &str) {
        let (x, y) =
            self.device
                .draw_string(self.x, self.y, vga_color::CHARLOTTE, text, self.start_x);
        self.x = x;
        self.y = y;
    }

    /// Logs a message to the BSOD screen.
    ///
    /// If the logger is called the first time it will draw the background and generic error heading text.
    pub fn log(&mut self, text: &str) {
        if !self.took_over {
            self.device.clear(vga_color::CLAY);
            self.__log(
                "OOPS - Something went wrong. Better check what it was using the stackframe:",
            );
            if self.x > 0 {
                self.x = LOGGER_START_INDENT_X;
                self.start_x = LOGGER_START_INDENT_X;
                self.y += LOGGER_START_INDENT_Y;
            }
            self.took_over = true;
        }
        self.__log(text);
    }

    /// Logs a message to the BSOD screen and adds a newline.
    pub fn logln(&mut self, text: &str) {
        self.log(text);
        if self.x > 0 {
            self.x = 0;
            self.y += CHAR_HEIGHT as u16;
        }
    }
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
    use core::fmt::Write;

    if let Some(logger) = LOGGER.lock().as_mut() {
        logger.write_fmt(args).unwrap();
    }
}

impl fmt::Write for Logger {
    /// This will never fail and can always be unwrapped.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.log(s);
        Ok(())
    }
}

#[macro_export]
/// Prints a string to the VGA buffer
macro_rules! log_print {
    ($($arg:tt)*) => ($crate::logger::__print(format_args!($($arg)*)));
}

#[macro_export]
/// Prints a string to the VGA buffer and appends a newline
macro_rules! log_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::log_print!("{}\n", format_args!($($arg)*)));
}
