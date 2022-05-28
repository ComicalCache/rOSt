#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, core_intrinsics)]

use alloc::{format, string::String};
use core::arch::asm;

extern crate alloc;
pub mod array_combiner;
pub mod byte_reader;
pub mod constants;
use crate::constants::{GIB, KIB, MIB};
pub mod port_extensions;
pub mod static_stack;
pub mod syscall_name;

/// Formats the size in bytes to a human readable string.
pub fn format_size(bytes: u64) -> String {
    match bytes {
        b if b < KIB => format!("{}B", b),
        b if b < MIB => format!("{}KiB", b / KIB),
        b if b < GIB => format!("{}MiB", b / MIB),
        b => format!("{}GiB", b / GIB),
    }
}

#[macro_export]
/// Macro for pushing all registers onto the stack.
macro_rules! push_all {
    () => {
        "push rax;push rbx;push rcx;push rdx;push rbp;push rsi;push rdi;push r8;push r9;push r10;push r11;push r12;push r13;push r14;push r15"
    };
}

#[macro_export]
/// Macro for popping all registers from the stack.
macro_rules! pop_all {
    () => {
        "pop r15;pop r14;pop r13;pop r12;pop r11;pop r10;pop r9;pop r8;pop rdi;pop rsi;pop rbp;pop rdx;pop rcx;pop rbx;pop rax"
    };
}

#[inline(always)]
/// Returns the current CPU tick. May be off a bit.
pub fn get_current_tick() -> u64 {
    let start_tick_low: u32;
    let start_tick_high: u32;
    unsafe {
        asm!(
            "rdtsc",
            out("eax")(start_tick_low),
            out("edx")(start_tick_high)
        );
    }
    u64::from(start_tick_low) | (u64::from(start_tick_high) << 32)
}

#[inline(always)]
/// Fast division by 255 using additions and shifts.
pub fn div_255_fast(x: u16) -> u8 {
    (((x) + (((x) + 257) >> 8)) >> 8) as u8
}
