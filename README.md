# x86-64 OS written in Rust 
This OS is based on the excellent blog of [phil-opp](https://os.phil-opp.com/). 

### Requirements
- [Rust](https://www.rust-lang.org/) using the nightly channel
- [QEMU](https://www.qemu.org/)
- [bootimage](https://crates.io/crates/bootimage) (installed via `cargo install bootimage`)

### Configuration

The entire build process is configured through the `target.json` and `.cargo/config.toml` files.

- `target.json`: configures the build target
- `.cargo/config.toml`: configures the build toolchain

## How to run
```bash
cargo run
```
will build the kernel and start up a qemu instance booting the kernel.

### Testing
Testing works using our own testing framework, located in the `test_framework` directory. When writing tests it's <u>important</u> to use the `serial_print!` and `serial_println!` macros for printing output. This is because the QEMU instance is hidden and running in the background, as well as exiting after all tests have been ran, sending all output via a serial port to the host machine's stdio. <u>Panics</u> while testing will be appropriately redirected to the serial port and <u>do not</u> require special macros.

All tests should be placed in the `tests` directory.

To run tests simply run `cargo test`.


### Troubleshooting
- If the build fails because of usage of unstable features, make sure that you enabled the nightly channel using `rustup default nightly` or try `rustup upgrade`


#### For more detailed descriptions and other references see [phil-opp's blog](https://os.phil-opp.com/).
