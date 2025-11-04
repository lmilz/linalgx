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
}

/// Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmn_map() {
        let mat = MatMN::new([[1, 2], [3, 4]]);
        let mapped = mat.map(|x| x * 2);
        assert_eq!(mapped, MatMN::new([[2, 4], [6, 8]]));
    }
}
