use crate::{
    algebra::{
        abstr::{Field, Scalar},
        linear::{matrix::CholeskyDec, Matrix},
    },
    elementary::Power,
};

impl<T> Matrix<T> where T: Field + Scalar + Power
{
    /// Decomposes the symetric, positive definite quadractic matrix A into a
    /// lower triangular matrix L A = L L^T
    ///
    /// # Arguments
    ///
    /// A has to be symetric and postive definite
    ///
    /// # Panics
    ///
    /// If A is not a quadratic matrix
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
    ///                                -1.0, 2.0, -1.0;
    ///                                 0.0, -1.0,  2.0];
    ///
    /// let l: (Matrix<f64>) = a.dec_cholesky().unwrap().l();
    /// # }
    /// ```
    pub fn dec_cholesky<'a>(self: &'a Self) -> Result<CholeskyDec<T>, ()>
    {
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n);
        self.dec_cholesky_r()
    }

    fn dec_cholesky_r<'a>(self: &'a Self) -> Result<CholeskyDec<T>, ()>
    {
        let (m, n) = self.dim();
        let exponent_sqrt: T = T::from_f64(0.5);
        let mut l: Matrix<T> = Matrix::zero(m, n);

        for i in 0..n
        {
            for j in 0..i + 1
            {
                let mut sum = T::zero();
                for k in 0..j
                {
                    sum += *l.get(i, k) * *l.get(j, k);
                }

                if i == j
                {
                    *l.get_mut(i, j) = (*self.get(i, i) - sum).pow(&exponent_sqrt)
                }
                else
                {
                    *l.get_mut(i, j) = (*self.get(i, j) - sum) / *l.get(j, j);
                }
            }
        }
        return Ok(CholeskyDec::new(l));
    }
}
