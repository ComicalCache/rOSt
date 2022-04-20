use core::fmt;

/// Colors text Red using the ANSI escape sequence.
pub struct Red(pub &'static str);

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[31m{}\x1B[0m", self.0)?;
        Ok(())
    }
}
