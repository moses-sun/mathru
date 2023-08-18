use crate::algebra::linear::matrix::General;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LowerTriangular<T> {
    pub(crate) matrix: General<T>,
}

impl<T> LowerTriangular<T> {
    pub fn new(matrix: General<T>) -> LowerTriangular<T> {
        LowerTriangular { matrix }
    }

    pub fn dim(&self) -> (usize, usize) {
        self.matrix.dim()
    }
}
