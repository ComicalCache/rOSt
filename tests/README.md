# Testing

Testing works using our own testing framework, located in the [src/test_framework](/src/test_framework) directory.

When writing tests it's <u>important</u> to use the `serial_print!` and `serial_println!` macros for printing output. This is because the QEMU instance is hidden and running in the background, as well as exiting after all tests have been ran, sending all output via a serial port to the host machine's stdio. <u>Panics</u> while testing will be appropriately redirected to the serial port and <u>do not</u> require special macros.

All tests should be placed in the [tests](/tests/) directory.

## How to run tests
To run all tests simply run `cargo t`.

## Setup tests

All test files require some boilerplate to work correctly. First create a file according to this template:

`tests/my_tests.rs`
```rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_core::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os_core::init();
    test_main();
    loop {}
}

// TODO: insert your tests bellow

#[test_case]
fn my_test_case() {
    // ...
}
```

Then go to [.cargo/config.toml](/.cargo/config.toml) and <u>append</u> the `t` alias with the new file you added:
```toml
# ...
[alias]
t = ["...", "--test", "my_tests"]
```

## Setup tests that fail

For setting up tests that fail we need to do things a bit differently. First create a file according to this template:

`tests/my_failing_test.rs`
```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use os_core::{
    ansi_colors::{Green, Red, Yellow},
    serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode},
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{} 1 {}", Yellow("Running"), Yellow("test(s):"));
     // TODO: change the names
    serial_println!("my_failing_test::my_failing_test_case...\t{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    my_failing_test_case();
     // TODO: change the names
    serial_println!("my_failing_test::my_failing_test_case...\t{}", Red("[failed]"));
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn my_failing_test_case() {
    // TODO: Implement your failing test case here
}
```

Then go to [.cargo/config.toml](/.cargo/config.toml) and <u>append</u> the `t` alias with the new file you added:
```toml
# ...
[alias]
t = ["...", "--test", "my_failing_test"]
```
Additionally you also need to add the following to [Cargo.toml](/Cargo.toml):
```toml
# ...

[[test]]
name = "my_failing_test"
harness = false

# ...
```
