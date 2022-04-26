use alloc::vec::Vec;

pub struct Combiner<'a, T: Sized + Default + Copy> {
    size: usize,
    data: Vec<&'a [T]>,
}

impl<'a, T: Sized + Default + Copy> Combiner<'a, T> {
    pub fn new() -> Self {
        Combiner {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn with(mut self, data: &'a [T]) -> Self {
        self.size += data.len();
        self.data.push(data);
        self
    }

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
