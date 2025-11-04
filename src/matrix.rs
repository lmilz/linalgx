use num_traits::{Float, Zero};
use std::ops::{Add, Mul, Sub};

/// A generic MxN-dimensional matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatMN<T, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}

impl<T: Copy, const M: usize, const N: usize> MatMN<T, M, N> {
    /// Creates a new MatMN from an array of values.
    pub fn new(data: [[T;M];N]) -> Self {
        Self { data }
    }
}
