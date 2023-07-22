use crate::algebra::abstr::{Complex, Real, Scalar};
use crate::algebra::linear::matrix::choleskydec::CholeskyDecomposition;
use crate::algebra::linear::matrix::{CholeskyDec, Diagonal};

impl<T> CholeskyDecomposition<T> for Diagonal<T>
where
    T: Real,
{
    /// Decomposes a symmetric, positive definite matrix $A$ into a product of
    /// a lower triangular matrix $L$ and its transpose such that $A = LL^T$.
    ///
    /// # Panics
    ///
    /// If the matrix $A$ is not quadratic or not positive definite.
    ///
    /// For efficiency reasons, the function may not check, if the matrix is
    /// symmetric, but just assume so.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{Diagonal, CholeskyDecomposition};
    ///
    /// let a: Diagonal<f64> = Diagonal::new(&[1.0; 128]);
    ///
    /// let l = a.dec_cholesky().unwrap().l();
    /// ```
    fn dec_cholesky(&self) -> Result<CholeskyDec<T>, String> {
        self.matrix.dec_cholesky()
    }
}

impl<T> CholeskyDecomposition<Complex<T>> for Diagonal<Complex<T>>
where
    T: Real,
    Complex<T>: Scalar,
{
    /// Decomposes a Hermitian, positive definite matrix $A$ into a product of
    /// a lower triangular matrix $L$ and its conjugate transpose, such that
    /// $A = LL^*$.
    ///
    /// # Panics
    ///
    /// If the matrix $A$ is not quadratic or not positive definite.
    ///
    /// For efficiency reasons, the function may not check if the matrix is
    /// Hermitian, but just assume so.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, LowerTriangular, CholeskyDecomposition};
    /// use mathru::matrix;
    ///
    /// let a: General<f64> = matrix![2.0, -1.0, 0.0;
    ///                               -1.0, 2.0, -1.0;
    ///                               0.0, -1.0, 2.0];
    ///
    /// let l: LowerTriangular<f64> = a.dec_cholesky().unwrap().l();
    /// ```
    fn dec_cholesky(&self) -> Result<CholeskyDec<Complex<T>>, String> {
        self.matrix.dec_cholesky()
    }
}
