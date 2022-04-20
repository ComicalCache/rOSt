use alloc::{boxed::Box, sync::Arc};
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;

pub trait Logger: Write + Send + Sync {
    fn log(&mut self, message: &str);
    fn logln(&mut self, message: &str);
}

lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Option<Box<dyn Logger>>>> = Arc::from(Mutex::new(None));
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
    if let Some(logger) = LOGGER.lock().as_mut() {
        (*logger).write_fmt(args).unwrap();
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
    () => ($crate::log_print!("\n"));
    ($($arg:tt)*) => ($crate::log_print!("{}\n", format_args!($($arg)*)));
}
