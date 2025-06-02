use std::ops::(Add, Mul, Sub)

/// A generic N-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecN<T, const N: usize> {
    pub data[T; N],
}

impl<T: Copy, const N: usize> VecN<T, N> {
    /// Creates a new VecN from an array of values.
    pub fn new (data: [T,N]) -> Self {
        Self { data }
    }
}