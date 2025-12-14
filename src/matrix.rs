use num_traits::Zero;
use std::ops::{Add, Mul, Sub};

/// A generic MxN-dimensional matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatMN<T, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}

impl<T: Copy, const M: usize, const N: usize> MatMN<T, M, N> {
    /// Creates a new MatMN from an array of values.
    pub fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    /// Applies a function to each element of the matrix
    pub fn map<U, F>(&self, f: F) -> MatMN<U, M, N>
    where
        F: Fn(T) -> U,
        U: Copy,
    {
        MatMN::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| f(self.data[i][j]))
        }))
    }

    /// Combines two matrices element-wise using a given binary function
    pub fn zip_with<U, R, F>(&self, other: MatMN<U, M, N>, f: F) -> MatMN<R, M, N>
    where
        F: Fn(T, U) -> R,
        U: Copy,
        R: Copy,
    {
        MatMN::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| f(self.data[i][j], other.data[i][j]))
        }))
    }

    /// Reduce the matrix to a single value using a folding function
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, T) -> B,
    {
        self.data.iter().flatten().copied().fold(init, f)
    }

    /// Matrix multiplication: m1 (M×N) * m2 (N×K) -> (M×K)
    pub fn mat_mul<const K: usize>(&self, other: MatMN<T, N, K>) -> MatMN<T, M, K>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Zero,
    {
        MatMN::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                (0..N).fold(T::zero(), |acc, k| acc + self.data[i][k] * other.data[k][j])
            })
        }))
    }

    /// Transpose matrix
    pub fn transpose(&self) -> MatMN<T, N, M> {
        MatMN::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| self.data[j][i])
        }))
    }
}

/// Element-wise matrix addition: m1 + m2
impl<T, const M: usize, const N: usize> Add for MatMN<T, M, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.zip_with(other, |a, b| a + b)
    }
}

/// Element-wise matrix subtraction: m1 - m2
impl<T, const M: usize, const N: usize> Sub for MatMN<T, M, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.zip_with(other, |a, b| a - b)
    }
}

/// Scalar multiplication: m * scalar
impl<T, const M: usize, const N: usize> Mul<T> for MatMN<T, M, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        self.map(|x| x * scalar)
    }
}

/// Matrix multiplication: m1 (M×N) * m2 (N×K) -> (M×K)
impl<T, const M: usize, const N: usize, const K: usize> Mul<MatMN<T, N, K>> for MatMN<T, M, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Zero,
{
    type Output = MatMN<T, M, K>;

    fn mul(self, other: MatMN<T, N, K>) -> MatMN<T, M, K> {
        self.mat_mul(other)
    }
}

/// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmn_map() {
        let mat = MatMN::new([[1, 2], [3, 4]]);
        let doubled = mat.map(|x| x * 2);
        assert_eq!(doubled, MatMN::new([[2, 4], [6, 8]]));
    }

    #[test]
    fn test_matmn_zip_with() {
        let mat1 = MatMN::new([[1, 2], [3, 4]]);
        let mat2 = MatMN::new([[5, 6], [7, 8]]);
        let result = mat1.zip_with(mat2, |a, b| a + b);
        assert_eq!(result, MatMN::new([[6, 8], [10, 12]]));
    }

    #[test]
    fn test_matmn_fold() {
        let mat = MatMN::new([[1, 2], [3, 4]]);
        let sum = mat.fold(0, |acc, x| acc + x);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_matmn_add() {
        let mat1 = MatMN::new([[1, 2], [3, 4]]);
        let mat2 = MatMN::new([[5, 6], [7, 8]]);
        let result = mat1 + mat2;
        assert_eq!(result, MatMN::new([[6, 8], [10, 12]]));
    }

    #[test]
    fn test_matmn_sub() {
        let mat1 = MatMN::new([[5, 6], [7, 8]]);
        let mat2 = MatMN::new([[1, 2], [3, 4]]);
        let result = mat1 - mat2;
        assert_eq!(result, MatMN::new([[4, 4], [4, 4]]));
    }

    #[test]
    fn test_matmn_scalar_mul() {
        let mat = MatMN::new([[1, 2], [3, 4]]);
        let result = mat * 2;
        assert_eq!(result, MatMN::new([[2, 4], [6, 8]]));
    }

    #[test]
    fn test_matmn_mul() {
        let mat1 = MatMN::new([[1, 2], [3, 4]]);
        let mat2 = MatMN::new([[5, 6], [7, 8]]);
        let result = mat1 * mat2;
        assert_eq!(result, MatMN::new([[19, 22], [43, 50]]));
    }

    #[test]
    fn test_matmn_transpose() {
        let mat = MatMN::new([[1, 2], [3, 4]]);
        let result = mat.transpose();
        assert_eq!(result, MatMN::new([[1, 3], [2, 4]]));
    }
}
