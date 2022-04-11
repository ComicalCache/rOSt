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


### Troubleshooting
- If the build fails because of usage of unstable features, make sure that you enabled the nightly channel using `rustup default nightly` or try `rustup upgrade`


#### For more detailed descriptions and other references see [phil-opp's blog](https://os.phil-opp.com/).
