use crate::algebra::linear::Matrix;
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Result of a cholesky decomposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CholeskyDec<T>
{
    l: Matrix<T>,
}

impl<T> CholeskyDec<T>
{
    pub fn new(m: Matrix<T>) -> CholeskyDec<T>
    {
        CholeskyDec { l: m }
    }
}

impl<T> CholeskyDec<T>
{
    /// Return the l matrix
    pub fn l(self: Self) -> Matrix<T>
    {
        return self.l;
    }
}
