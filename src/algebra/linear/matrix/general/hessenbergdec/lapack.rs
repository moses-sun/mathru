use crate::{
    algebra::{
        abstr::{Field, Scalar},
        linear::matrix::{General, HessenbergDec, HessenbergDecomposition, UpperHessenberg},
    },
    elementary::Power,
};

impl<T> HessenbergDecomposition<T> for General<T>
where
    T: Field + Scalar + Power,
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
    /// use mathru::algebra::linear::matrix::{General, UpperHessenberg, HessenbergDecomposition};
    ///
    /// let a: General<f64> = General::new(3, 3, vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);
    /// let (q, h): (General<f64>, UpperHessenberg<f64>) = a.dec_hessenberg().qh();
    /// ```
    fn dec_hessenberg(&self) -> HessenbergDec<T> {
        let (m, n): (usize, usize) = self.dim();
        debug_assert_eq!(
            m, n,
            "Unable to compute the hessenberg decomposition of a non-square matrix"
        );
        debug_assert_ne!(
            m, 0,
            "Unable to compute the hessenberg decomposition of an empty matrix."
        );
        let (m, n): (usize, usize) = self.dim();

        //lapack(fortran) uses column major order
        let mut self_data = self.clone().data;
        let n_i32: i32 = n as i32;

        let mut tau: Vec<T> = vec![T::zero(); n - 1];

        let mut info: i32 = 0;

        let lwork: i32 = T::xgehrd_work_size(
            n_i32,
            1,
            n_i32,
            &mut self_data[..],
            n_i32,
            tau.as_mut(),
            &mut info,
        );

        debug_assert_eq!(0, info);

        let mut work_xgehrd: Vec<T> = vec![T::zero(); lwork as usize];

        T::xgehrd(
            n_i32,
            1,
            n_i32,
            &mut self_data[..],
            n_i32,
            tau.as_mut(),
            &mut work_xgehrd[..],
            lwork,
            &mut info,
        );

        debug_assert_eq!(0, info);

        let h: General<T> = General::new(n, n, self_data.clone()).h();
        let mut q = self_data;

        let mut info: i32 = 0;

        let lwork: i32 =
            T::xorghr_work_size(n_i32, 1, n_i32, &mut q[..], n_i32, tau.as_mut(), &mut info);
        let mut work_xorghr = vec![T::zero(); lwork as usize];

        debug_assert_eq!(0, info);

        T::xorghr(
            n_i32,
            1,
            n_i32,
            &mut q[..],
            n_i32,
            &tau[..],
            &mut work_xorghr[..],
            lwork,
            &mut info,
        );

        debug_assert_eq!(0, info);

        HessenbergDec::new(General::new(m, n, q), UpperHessenberg::new(h))
    }
}

impl<T> General<T>
where
    T: Field + Scalar + Power,
{
    fn h(mut self) -> Self {
        let (m, _n) = self.dim();
        for i in 2..m {
            for k in 0..(i - 1) {
                self[[i, k]] = T::zero();
            }
        }
        self
    }
}
