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

    /// Combines two vectors element-wise using a given binary function (zipwith pattern in functional programming)
    pub fn zip_with<U, R, F>(&self, other: VecN<U, N>, f: F) -> VecN<R, N> 
    where
        F: Fn(T, U) -> R,
        U: Copy, 
        R: Copy
    {
        VecN::new(std::array::from_fn(|i| f(self.data[i], other.data[i])))
    }

    /// Reduce the vector to a single value using a folding function
    pub fn fold<B, F>(&self, init: B, f: F) -> B 
    where 
        F: Fn(B, T) -> B
    {
        self.data.iter().copied().fold(init, f)
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

    #[test]
    fn test_zip_with() {
        let a = VecN::new([1, 2, 3]);
        let b = VecN::new([4, 5, 6]);
        let result = a.zip_with(b, |x,y| x + y);
        assert_eq!(result, VecN::new([5, 7, 9]));
    }

    #[test]
    fn test_fold() {
        let v = VecN::new([1, 2, 3]);
        let sum = v.fold(0, |acc, x| acc+x);
        assert_eq!(sum, 6);
    }
}