# x86-64 OS written in Rust 
This OS is based on the excellent blog of [phil-opp](https://os.phil-opp.com/). 

### Structure
TODO

### Requirements
- [Rust](https://www.rust-lang.org/) using the nightly channel
- [QEMU](https://www.qemu.org/)
- [llvm-tools-preview](https://docs.rs/llvm-tools/latest/llvm_tools/) (installed via `rustup component add llvm-tools-preview`)

## How to run
```bash
cargo krun
```
will build the kernel and start up a qemu instance booting the kernel.

### Testing
TODO


### Troubleshooting
- If the build fails because of usage of unstable features, make sure that you enabled the nightly channel using `rustup default nightly` or try `rustup upgrade`
