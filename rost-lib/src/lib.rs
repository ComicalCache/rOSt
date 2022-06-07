#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, core_intrinsics, alloc_error_handler)]

use core::arch::asm;
extern crate alloc;

mod thread_utils;

pub fn initialize_syscalls() {
    use kernel::syscalls::register_syscall;
    use kernel::syscalls::SysCallName;
    register_syscall(SysCallName::ThreadExit as u16, thread_utils::thread_exit);
}

#[inline(always)]
pub fn syscall(name: u64, arg1: u64, arg2: u64) -> u64 {
    unsafe {
        let result: u64;
        asm!(
            "push r10; push r11; push rcx",
            "syscall",
            "pop rcx; pop r11; pop r10",
            in("rdi")(name),
            in("rsi")(arg1),
            in("rdx")(arg2),
            out("rax")(result)
        );
        result
    }
}
