#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, core_intrinsics)]

use alloc::{format, string::String};

extern crate alloc;
pub mod array_combiner;
pub mod byte_reader;
pub mod port_extensions;
pub mod static_stack;

pub const KIBIBYTE: u64 = 1024;
pub const MEBIBYTE: u64 = KIBIBYTE * 1024;
pub const GIBIBYTE: u64 = MEBIBYTE * 1024;

pub fn format_size(bytes: u64) -> String {
    match bytes {
        b if b < KIBIBYTE => format!("{}B", b),
        b if b < MEBIBYTE => format!("{}KiB", b / KIBIBYTE),
        b if b < GIBIBYTE => format!("{}MiB", b / MEBIBYTE),
        b => format!("{}GiB", b / GIBIBYTE),
    }
}
