#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(
    option_get_or_insert_default,
    abi_x86_interrupt,
    generic_const_exprs,
    core_intrinsics,
    asm_const,
    naked_functions
)]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc};
use lazy_static::lazy_static;
use spin::Mutex;

mod init;
pub use init::{hlt_loop, init, register_driver, reload_drivers};

use crate::logger::Logger;

mod debug;
mod interrupts;
pub mod logger;
mod memory;
pub mod processes;
pub mod syscalls;

lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Option<Box<dyn Logger>>>> = Arc::from(Mutex::new(None));
}
