// conditional compilation, only included on `cargo test`, else discarded. 
// Avoids compiler warnings about unused code
#[cfg(test)]
pub mod trivial_test;