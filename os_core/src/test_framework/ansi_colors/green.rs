use core::fmt;

pub struct Green(pub &'static str);

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x1B[32m{}\x1B[0m", self.0)?;
        Ok(())
    }
}