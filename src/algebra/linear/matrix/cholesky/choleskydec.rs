//! Cholesky decomposition
//!
//! The Cholesky decomposition of a Hermitian positive definite matrix $A$ is
//! a product of a lower triangular matrix $L$ and its conjugate transpose.
//! Expressed in symbols that is $A = LL^*$.
//!
//! Here is an example on how to compute the Cholesky decomposition of a real
//! valued matrix:
//! ```
//! # #[macro_use]
//! # extern crate mathru;
//! # fn main() -> Result<(), String> {
//! use mathru::algebra::linear::Matrix;
//! use mathru::matrix;
//! let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
//!                                -1.0, 2.0, -1.0;
//!                                 0.0, -1.0,  2.0];
//! let l: Matrix<f64> = a.dec_cholesky()?.l();
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::algebra::linear::Matrix;
use std::clone::Clone;

/// Result of a Cholesky decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CholeskyDec<T> {
    l: Matrix<T>,
}

impl<T> CholeskyDec<T> {
    /// Initializes a Cholesky decomposition with a given matrix.
    ///
    /// The parameter `m` should be a lower triangular matrix defining the
    /// Cholesky decomposition.
    pub fn new(m: Matrix<T>) -> CholeskyDec<T> {
        CholeskyDec { l: m }
    }
}

impl<T> CholeskyDec<T> {
    /// Returns the lower triangular matrix.
    pub fn l(self) -> Matrix<T> {
        self.l
    }
}
