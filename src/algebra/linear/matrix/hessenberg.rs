use algebra::linear::{Vector, Matrix};
use algebra::abstr::Zero;
use algebra::abstr::Real;



impl<T> Matrix<T>
     where T: Real
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
    /// q:
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(3, 3, vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);
    /// let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg();
    ///
    /// ```
    pub fn dec_hessenberg(self: &Self) -> (Matrix<T>, Matrix<T>)
    {
        let (m, n) : (usize, usize) = self.dim();
        assert!(m == n, "Unable to compute the hessenberg decompositoin of a non-square matrix");
        assert!(m != 0, "Unable to compute the hessenberg decomposition of an empty matrix.");
        self.dec_hessenberg_r()
    }

    #[cfg(feature = "native")]
    fn dec_hessenberg_r(self: &Self) -> (Matrix<T>, Matrix<T>)
    {
        let (m, n) : (usize, usize) = self.dim();

        let mut q: Matrix<T> = Matrix::one(m);
        let mut h: Matrix<T> = self.clone();

        for k in 1..m
        {
            let v: Vector<T> = h.get_column(&(k-1));

            let househ: Matrix<T> = Matrix::householder(&v, k);
            h = &househ * &h;
            q = &househ * &q;
            h = &h.clone() * &househ.transpose_inplace();
        }

        return (q.transpose_inplace(), h);
    }

    #[cfg(feature = "lapack_be")]
    fn dec_hessenberg_r(self: &Self) -> (Matrix<T>, Matrix<T>)
    {
        let (m, n) : (usize, usize) = self.dim();

        let mut self_data = self.data.clone();
        let n_i32: i32 = n as i32;

        let mut tau: Vec<T> = vec![Zero::zero(); n - 1];

        let mut info: i32 = 0;

        let lwork: i32 = T::xgehrd_work_size(n_i32, 1, n_i32, & mut self_data[..], n_i32, tau.as_mut(), &mut info);

        assert_eq!(0, info);

        let mut work: Vec<T> = vec![Zero::zero(); lwork as usize];

        T::xgehrd(
            n_i32,
            1,
            n_i32,
            &mut self_data[..],
            n_i32,
            tau.as_mut(),
            work.as_mut(),
            lwork,
            &mut info,
        );

        assert_eq!(0, info);

        let h: Matrix<T> = Matrix::new(n, n, self_data.clone()).h();
        let mut q = self_data;

        let mut info: i32 = 0;

        let lwork: i32 = T::xorghr_work_size(n_i32, 1, n_i32, &mut q[..], n_i32, tau.as_mut(), &mut info);
        //let mut work = vec![Zero::zero(), lwork as usize];

        assert_eq!(0, info);

        T::xorghr(
            n_i32,
            1,
            n_i32,
            &mut q[..],
            n_i32,
            &tau[..],
            &mut work[..],
            lwork,
            &mut info,
        );

        assert_eq!(0, info);


        //return (q.transpose_inplace(), h);
        (Matrix::new(m, n, q), h)
    }

    #[cfg(feature = "lapack_be")]
    fn h(mut self: Self) -> Self
    {
        let (m, _n) = self.dim();
        for i in 2..m
        {
            for k in 0..i
            {
                *self.get_mut(&i, &k) = T::zero();
            }
        }
        self
    }
}



