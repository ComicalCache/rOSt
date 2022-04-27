use core::fmt;

/// Colors text Yellow using the ANSI escape sequence.
pub struct Yellow(pub &'static str);

impl fmt::Display for Yellow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[93m{}\x1B[0m", self.0)?;
        Ok(())
    }
}
