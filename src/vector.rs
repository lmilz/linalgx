use std::ops::{Add, Mul, Sub};

/// A generic N-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecN<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Copy, const N: usize> VecN<T, N> {
    /// Creates a new VecN from an array of values.
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }

    /// Applies a function to each element of the vector (map function in functional programming)
    pub fn map<U, F>(&self, f: F) -> VecN<U, N> 
    where
        F: Fn(T) -> U,
        U: Copy
    {
        VecN::new(std::array::from_fn(|i| f(self.data[i])))
    }    
}

/// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let v = VecN::new([1,2,3]);
        let doubled = v.map(|x| 2*x);
        assert_eq!(doubled, VecN::new([2,4,6]));
    }
}