use crate::algebra::{
    abstr::{Field, Scalar, Zero},
    linear::matrix::General,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Diagonal matrix
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Diagonal<T> {
    pub(crate) matrix: General<T>,
}

impl<T> Diagonal<T>
where
    T: Field + Scalar + Zero,
{
    /// Construct a matrix with vec as its diagonal.
    pub fn new(vec: &Vec<T>) -> Diagonal<T> {
        let mut g = General::zero(vec.len(), vec.len());
        for (idx, v) in vec.iter().enumerate() {
            g[[idx, idx]] = *v;
        }
        Diagonal { matrix: g }
    }

    pub fn dim(&self) -> (usize, usize) {
        self.matrix.dim()
    }
}
