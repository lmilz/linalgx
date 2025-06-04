/// A generic MxN-dimensional matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatMN<T, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}