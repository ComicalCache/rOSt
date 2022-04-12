use volatile::Volatile;

use super::screen_char::ScreenChar;

// default text buffer dimensions
pub const TEXT_BUFFER_HEIGHT: usize = 25;
pub const TEXT_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub(crate) struct VgaTextBuffer {
    pub(crate) chars: [[Volatile<ScreenChar>; TEXT_BUFFER_WIDTH]; TEXT_BUFFER_HEIGHT],
}