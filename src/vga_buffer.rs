use core::ops::{Deref, DerefMut};

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::offsets;

#[macro_export]
/// Prints a string to the VGA buffer
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::__print(format_args!($($arg)*)));
}

#[macro_export]
/// Prints a string to the VGA buffer and appends a newline
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
    use core::fmt::Write;

    // unwrap should be safe because Writer::write_str is safe
    VGA_TEXT_BUFFER_WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    /// Global VGA text buffer writer, locked by a spinmutex
    pub static ref VGA_TEXT_BUFFER_WRITER: Mutex<VgaTextBufferWriter> = Mutex::new(VgaTextBufferWriter {
        text_buffer_height: TEXT_BUFFER_HEIGHT,
        text_buffer_width: TEXT_BUFFER_WIDTH,

        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(offsets::VGA_BUFFER as *mut VgaTextBuffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl Deref for ScreenChar {
    type Target = ScreenChar;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

// default text buffer dimensions
const TEXT_BUFFER_HEIGHT: usize = 25;
const TEXT_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VgaTextBuffer {
    chars: [[Volatile<ScreenChar>; TEXT_BUFFER_WIDTH]; TEXT_BUFFER_HEIGHT],
}

pub struct VgaTextBufferWriter {
    text_buffer_height: usize,
    text_buffer_width: usize,

    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut VgaTextBuffer,
}

/// Internal VgaTextBufferWriter implementations
impl VgaTextBufferWriter {
    /// Internal implementation of writing to the VGA text buffer, scrolling if necessary
    fn __write_byte(&mut self, byte: u8) {
        if self.column_position >= self.text_buffer_width {
            self.__new_line();
        }

        let row = self.row_position;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code,
        });

        self.column_position += 1;
    }

    /// Writes a new line to the VGA text buffer, scrolling if the cursor is at the bottom of the buffer
    fn __new_line(&mut self) {
        self.row_position += 1;

        if self.row_position == self.text_buffer_height {
            self.__scroll();
            self.__clear_row(self.text_buffer_height - 1);
            self.row_position = self.text_buffer_height - 1;
        }

        self.column_position = 0;
    }

    /// Scrolls the VGA text buffer up by one line
    fn __scroll(&mut self) {
        for row in 1..self.text_buffer_height {
            for col in 0..self.text_buffer_width {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
    }

    /// Clears a row in the VGA text buffer
    fn __clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..self.text_buffer_width {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Sets the cursor position to the bottom of the VGA text buffer for outputting panic messages and the colour
    /// code for the panic messages
    pub fn __set_panic_config(&mut self) {
        self.row_position = self.text_buffer_height - 1;
        self.column_position = 0;
        self.color_code = ColorCode::new(Color::LightRed, Color::Black);
        self.__new_line();
    }
}

#[allow(dead_code)]
/// VgaTextBufferWriter interface implementations for writing to the VGA text buffer
impl VgaTextBufferWriter {
    /// writes raw bytes to the VGA text buffer
    pub fn write_raw_byte(&mut self, byte: u8) {
        self.__write_byte(byte);
    }

    /// writes a byte to the VGA text buffer or a new line if the byte is a newline.
    ///
    /// If the newline is at the bottom of the VGA text buffer, the buffer is scrolled up.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.__new_line(),
            byte => self.__write_byte(byte),
        }
    }

    /// Writes a string to the VGA text buffer
    pub fn write_string<S: AsRef<str>>(&mut self, s: S) {
        for byte in s.as_ref().bytes() {
            match byte {
                // printable byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable range, instead print `■`
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Writes the raw string to the VGA text buffer
    pub fn write_raw_string<S: AsRef<str>>(&mut self, s: S) {
        for byte in s.as_ref().bytes() {
            self.write_raw_byte(byte);
        }
    }
}

#[allow(dead_code)]
/// VgaTextBufferWriter interface implementations for configuring the VgaTextBufferWriter
impl VgaTextBufferWriter {
    /// Sets the cursor position to the specified row and column
    pub fn set_pos(&mut self, row: usize, col: usize) {
        if row >= self.text_buffer_height || col >= self.text_buffer_width {
            // TODO logger
            panic!("VgaTextBufferWriter::set_pos: row or col out of bounds");
        }

        self.row_position = row;
        self.column_position = col;
    }

    /// Sets the color code for the VgaTextBufferWriter
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
}

impl fmt::Write for VgaTextBufferWriter {
    /// Write a formatted string to the VGA text buffer.
    ///
    /// This will never fail and can always be unwrapped.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
