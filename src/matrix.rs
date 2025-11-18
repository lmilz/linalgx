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
}

/// Unit Test
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
}
