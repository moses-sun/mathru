use crate::algebra::abstr::Zero;
use crate::algebra::abstr::{Complex, Real, Scalar};
use crate::{
    algebra::linear::{matrix::CholeskyDec, Matrix},
    elementary::Power,
};

impl<T> Matrix<T>
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
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main() -> Result<(), String> {
    /// use mathru::algebra::linear::Matrix;
    /// use mathru::matrix;
    /// let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
    ///                                -1.0, 2.0, -1.0;
    ///                                 0.0, -1.0,  2.0];
    /// let l: Matrix<f64> = a.dec_cholesky()?.l();
    /// # Ok(())
    /// # }
    /// ```
    pub fn dec_cholesky(&self) -> Result<CholeskyDec<T>, String> {
        let (m, n) = self.dim();
        assert_eq!(m, n);

        let mut l: Matrix<T> = Matrix::zero(m, n);

        for i in 0..n {
            for j in 0..i + 1 {
                let mut sum = T::zero();
                for k in 0..j {
                    sum += l[[i, k]] * l[[j, k]];
                }

                if i == j {
                    assert!(
                        self[[i, i]] - sum >= T::zero(),
                        "The matrix is not positive definite."
                    );
                    l[[i, j]] = (self[[i, i]] - sum).sqrt();
                } else {
                    assert_ne!(l[[j, j]], T::zero(), "The matrix is not positive definite");
                    l[[i, j]] = (self[[i, j]] - sum) / l[[j, j]];
                }
            }
        }
        Ok(CholeskyDec::new(l))
    }
}

impl<T> Matrix<Complex<T>>
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
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main() -> Result<(), String> {
    /// use mathru::algebra::abstr::Complex;
    /// use mathru::algebra::linear::Matrix;
    /// use mathru::matrix;
    /// let a = matrix![Complex::new(2.0,  0.0), Complex::new(0.0, 1.0);
    ///                 Complex::new(0.0, -1.0), Complex::new(2.0, 0.0)];
    /// let l = a.dec_cholesky()?.l();
    /// # Ok(())
    /// # }
    /// ```
    pub fn dec_cholesky(&self) -> Result<CholeskyDec<Complex<T>>, String> {
        let (m, n) = self.dim();
        assert_eq!(m, n);

        let mut l: Matrix<Complex<T>> = Matrix::zero(m, n);

        for i in 0..n {
            for j in 0..i + 1 {
                let mut sum = Complex::<T>::zero();
                for k in 0..j {
                    sum += l[[i, k]] * l[[j, k]].conj();
                }

                if i == j {
                    assert!(
                        (self[[i, i]] - sum).re >= T::zero(),
                        "The matrix is not positive definite."
                    );
                    assert!(
                        (self[[i, i]] - sum).im == T::zero(),
                        "The matrix is not Hermitian."
                    );
                    l[[i, j]] = (self[[i, i]] - sum).sqrt();
                } else {
                    assert_ne!(
                        l[[j, j]],
                        Complex::new(T::zero(), T::zero()),
                        "The matrix is not positive definite"
                    );
                    l[[i, j]] = (self[[i, j]] - sum) / l[[j, j]];
                }
            }
        }
        Ok(CholeskyDec::new(l))
    }
}
