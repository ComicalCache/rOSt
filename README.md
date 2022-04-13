# x86-64 OS written in Rust 
This OS is based on the excellent blog of [phil-opp](https://os.phil-opp.com/). 

### Structure
The os core is a library called [os_core](/src/os_core.rs) which has all the important functions and logic of the OS.

This library is used for testing the components in integration tests.

The runnable OS is built as binary called [os](/src/os.rs).

### Requirements
- [Rust](https://www.rust-lang.org/) using the nightly channel
- [QEMU](https://www.qemu.org/)
- [bootimage](https://crates.io/crates/bootimage) (installed via `cargo install bootimage`)
- [llvm-tools-preview](https://docs.rs/llvm-tools/latest/llvm_tools/) (installed via `rustup component add llvm-tools-preview`)

### Configuration

The entire build process is configured through the [target.json](/target.json), [Cargo.toml](/Cargo.toml) and [.cargo/config.toml](/.cargo/config.toml) files.

- [target.json](/target.json): configures the build target
- [Cargo.toml](/Cargo.toml): configures testing and manages dependencies
- [.cargo/config.toml](/.cargo/config.toml): configures the build toolchain

## How to run
```bash
cargo run
```
will build the kernel and start up a qemu instance booting the kernel.

### Testing
Testing works using our own testing framework, located in [lib.rs](/src/lib.rs) and the [src/test_framework](/src/test_framework) directory.

When writing tests it's <u>important</u> to use the `serial_print!` and `serial_println!` macros for printing output. This is because the QEMU instance is hidden and running in the background, as well as exiting after all tests have been ran, sending all output via a serial port to the host machine's stdio. <u>Panics</u> while testing will be appropriately redirected to the serial port and <u>do not</u> require special macros.

All tests should be placed in the [tests](/tests/) directory.

#### Setup tests

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

// ! insert your tests bellow

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

To run all tests simply run `cargo t`.


### Troubleshooting
- If the build fails because of usage of unstable features, make sure that you enabled the nightly channel using `rustup default nightly` or try `rustup upgrade`


#### For more detailed descriptions and other references see [phil-opp's blog](https://os.phil-opp.com/).
