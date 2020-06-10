use crate::{
    algebra::{
        abstr::{Field, Scalar},
        linear::{matrix::HessenbergDec, Matrix, Vector},
    },
    elementary::Power,
};

impl<T> Matrix<T> where T: Field + Scalar + Power
{
    /// Decomposes self in to the M
    ///
    /// q * h * q^T = self
    ///
    /// # Arguments
    ///
    /// # Return
    ///
    /// (q, h)
    ///
    /// # Panics
    ///
    /// if M is not a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(3, 3, vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);
    /// let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg().qh();
    /// ```
    pub fn dec_hessenberg(self: &Self) -> HessenbergDec<T>
    {
        let (m, n): (usize, usize) = self.dim();
        assert!(m == n,
                "Unable to compute the hessenberg decompositoin of a non-square matrix");
        assert!(m != 0,
                "Unable to compute the hessenberg decomposition of an empty matrix.");
        return self.dec_hessenberg_r();
    }

    fn dec_hessenberg_r(self: &Self) -> HessenbergDec<T>
    {
        let (m, _n): (usize, usize) = self.dim();

        let mut q: Matrix<T> = Matrix::one(m);
        let mut h: Matrix<T> = self.clone();

        for k in 1..m
        {
            let v: Vector<T> = h.get_column(k - 1);

            let househ: Matrix<T> = Matrix::householder(&v, k);
            h = &househ * &h;
            q = &househ * &q;
            h = &h.clone() * &househ.transpose();
        }

        return HessenbergDec::new(q.transpose(), h);
    }
}
