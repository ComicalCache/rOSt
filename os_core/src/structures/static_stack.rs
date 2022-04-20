pub struct StaticStack<T: Sized + Default + Clone + Copy, const C: usize> {
    buffer: [T; C],
    top: usize,
}

impl<T: Sized + Default + Clone + Copy, const C: usize> StaticStack<T, C> {
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); C],
            top: 0,
        }
    }

    pub fn push(&mut self, item: &T) -> Result<(), &str> {
        if self.top == C {
            return Err("Static stack is full");
        }
        self.buffer[self.top] = item.clone();
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.buffer[self.top])
    }

    pub fn length(&self) -> usize {
        self.top
    }
}
