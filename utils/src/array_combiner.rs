use alloc::vec::Vec;

/// A struct that combines multiple arrays into one.
pub struct Combiner<'a, T: Sized + Default + Copy> {
    size: usize,
    data: Vec<&'a [T]>,
}

impl<'a, T: Sized + Default + Copy> Combiner<'a, T> {
    /// Creates a new `Combiner`.
    pub fn new() -> Self {
        Combiner {
            size: 0,
            data: Vec::new(),
        }
    }

    /// Adds a new array to the combiner.
    pub fn with(mut self, data: &'a [T]) -> Self {
        self.size += data.len();
        self.data.push(data);
        self
    }

    /// Combines all arrays into one.
    ///
    /// ## Returns
    /// A result containing all arrays combined in `[T; S]` or None if `S` does not equal the combiners size.
    pub fn build<const S: usize>(self) -> Option<[T; S]> {
        if self.size != S {
            return None;
        }
        let mut result = [T::default(); S];
        let mut start: usize = 0;
        for data in self.data {
            result[start..start + data.len()].copy_from_slice(data);
            start += data.len();
        }
        Some(result)
    }
}

impl<'a, T: Sized + Default + Copy> Default for Combiner<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
