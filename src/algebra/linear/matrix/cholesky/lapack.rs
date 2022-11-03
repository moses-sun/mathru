use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::CholeskyDec, Matrix},
};

impl<T> Matrix<T>
where
    T: Field + Scalar,
{
    /// Decomposes a Hermitian, positive definite matrix $A$ into a product of
    /// a lower triangular matrix $L$ and its conjugate transpose such that
    /// $A = LL^T$.
    ///
    /// # Panics
    ///
    /// If the matrix $A$ is not quadratic or not positive definite.
    ///
    /// For efficiency reasons, the function may not check, if the matrix is
    /// Hermitian, but just assume so.
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
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n);

        let (_m, n) = self.dim();
        let n_i32: i32 = n as i32;

        let mut info: i32 = 0;

        let mut l_data: Vec<T> = self.clone().data;

        T::xpotrf('L', n_i32, l_data.as_mut_slice(), n_i32, &mut info);

        if info < 0 {
            return Err(String::from("LAPACK reported illegal argument."));
        }

        let mut l: Matrix<T> = Matrix::new(n, n, l_data);

        //fill above diagonal with zeros
        for i in 0..n {
            for j in (i + 1)..n {
                l[[i, j]] = T::zero();
            }
        }

        return Ok(CholeskyDec::new(l));
    }
}
