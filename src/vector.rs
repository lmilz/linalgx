use std::ops::{Add, Mul, Sub};

/// A generic N-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecN<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Copy, const N: usize> VecN<T, N> {
    /// Creates a new VecN from an array of values.
    pub fn new (data: [T; N]) -> Self {
        Self { data }
    }

    /// Applies a function to each element of the vector (map function in functional programming)
    pub fn map<U, F: Fn(T) -> U>(&self, f: F) -> VecN<U, N> {
        VecN::new(std::array::from_fn(|i| f(self.data[i])))
    }
}