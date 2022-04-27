# x86-64 OS written in Rust

This OS is based on the excellent blog of [phil-opp](https://os.phil-opp.com/).

### Structure

The os core is a library called [os_core](/src/os_core.rs) which has all the important functions and logic of the OS.

This library is used for testing the components in integration tests.

The runnable OS is built as binary called [os](/src/os.rs).

### Requirements

- [Rust](https://www.rust-lang.org/) using the nightly channel
- [QEMU](https://www.qemu.org/)
- [llvm-tools-preview](https://docs.rs/llvm-tools/latest/llvm_tools/) (installed via `rustup component add llvm-tools-preview`)

### Configuration

The entire build process is configured through the [target.json](/target.json), [Cargo.toml](/Cargo.toml) and [.cargo/config.toml](/.cargo/config.toml) files.

- [target.json](/target.json): configures the build target
- [Cargo.toml](/Cargo.toml): configures testing and manages dependencies
- [.cargo/config.toml](/.cargo/config.toml): configures the build toolchain

## How to run

```bash
cargo krun
```

will build the kernel and start up a qemu instance booting the kernel.

### Testing

Look at [integration testing](/tests/) for more information.

### Troubleshooting

- If the build fails because of usage of unstable features, make sure that you enabled the nightly channel using `rustup default nightly` or try `rustup upgrade`

#### For more detailed descriptions and other references see [phil-opp's blog](https://os.phil-opp.com/).

<a href="https://iconscout.com/icons/processor-chip" target="_blank">Processor Chip Icon</a> by <a href="https://iconscout.com/contributors/kolo-design" target="_blank">Kalash</a>
