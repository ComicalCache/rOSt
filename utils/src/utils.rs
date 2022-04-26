#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, core_intrinsics)]

use alloc::{format, string::String};

extern crate alloc;
pub mod array_combiner;
pub mod port_extensions;
pub mod static_stack;

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!(" {}B", bytes);
    }
    if bytes < 1024 * 1024 {
        return format!(" {}KiB", bytes / 1024);
    }
    if bytes < 1024 * 1024 * 1024 {
        return format!(" {}MiB", bytes / 1024 / 1024);
    }
    format!(" {}GiB", bytes / 1024 / 1024 / 1024)
}
