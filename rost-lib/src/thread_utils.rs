use internal_utils::serial_println;

pub extern "C" fn thread_exit(code: u64, _: u64) -> u64 {
    serial_println!("thread_exit: code={}", code);
    0
}
