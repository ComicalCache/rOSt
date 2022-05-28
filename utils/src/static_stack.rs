/// A stack with a fixed size.
pub struct StaticStack<T: Sized + Default + Clone + Copy, const C: usize> {
    buffer: [T; C],
    top: usize,
}

/// The error type for the `StaticStack` struct.
#[derive(Debug)]
#[repr(u8)]
pub enum StaticStackError {
    /// The stack is full.
    EOS,
}

impl<T: Sized + Default + Clone + Copy, const C: usize> StaticStack<T, C> {
    /// Creates a new `StaticStack` of capacity `C` and type `T`.
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); C],
            top: 0,
        }
    }

    /// Pushes a value onto the stack.
    ///
    /// ## Returns
    /// Returns `Ok(())` if the operation was successful, and `Err(StaticStackError::EOS)` if the stack is full.
    pub fn push(&mut self, item: &T) -> Result<(), StaticStackError> {
        if self.top == C {
            return Err(StaticStackError::EOS);
        }
        self.buffer[self.top] = item.clone();
        self.top += 1;
        Ok(())
    }

    /// Pops a value off the stack.
    ///
    /// ## Returns
    /// Returns `Some(T)` if the operation was successful, and `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.buffer[self.top])
    }

    /// Returns the number of elements in the stack.
    pub fn length(&self) -> usize {
        self.top
    }
}
