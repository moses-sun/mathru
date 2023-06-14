use crate::algebra::linear::matrix::{General, LowerTriangular};

impl<T> From<General<T>> for LowerTriangular<T> {
    fn from(matrix: General<T>) -> Self {
        LowerTriangular { matrix }
    }
}
