use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Real};
#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Zero};

impl<T> Matrix<T>
     where T: Real
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    ///
    /// ```
    pub fn inv<'a>(self: &'a Self) -> Result<Matrix<T>, ()>
    {
        self.inv_r()
    }

    #[cfg(feature = "native")]
    pub fn inv_r(self: & Self) -> Result<Matrix<T>, ()>
    {
        let (mut l, mut u, p) : (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();

        l.subst_forward();
        u.subst_backward();
        return Ok(&(&u * &l) * &p);
    }


    #[cfg(feature = "native")]
    ///
    /// inplace backward substitution
    ///
    fn subst_backward<'a>(self: &'a mut Self)
    {
        for k in (0..self.n).rev()
        {
            for l in (k..self.n).rev()
            {

                let mut sum : T = T::zero();

                for i in (k+1)..self.n
                {
                    sum += self.data[i * self.m + k] * self.data[l * self.m + i];
                }

                let b : T;
                if k == l
                {
                    b = T::one();

                }
                else
                {
                    b = T::zero();
                }
                let div : T = self.data[k * self.m + k];
                self.data[l * self.m + k] = (b - sum) / div;

            }
        }
    }

    #[cfg(feature = "native")]
    ///
    /// inplace forward substitution
    ///
    fn subst_forward<'a>(self: &'a mut Self)
    {

        for k in 0..self.n
        {
            for l in 0..k
            {

                let mut sum : T = T::zero();

                for i in 0..k
                {
                    sum += self.data[i * self.m + k] * self.data[l * self.m + i];
                }

                let b : T;
                if k == l
                {
                    b = T::one();

                }
                else
                {
                    b = T::zero();
                }
                let div : T = self.data[k * self.m + k];
                self.data[l * self.m + k] = (b - sum) / div;

            }
        }
    }

    #[cfg(feature = "blaslapack")]
    pub fn inv_r(self: & Self) -> Result<Matrix<T>, ()>
    {
         let (m, n): (usize, usize) = self.dim();
        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;

        let dim_min: i32= m_i32.min(n_i32);
        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];

        let mut info: i32 = 0;

        let mut self_data = self.transpose().data;

        T::xgetrf(
            m_i32,
            n_i32,
            self_data.as_mut_slice(),
            m_i32,
            ipiv.as_mut_slice(),
            &mut info,
        );

        assert_eq!(0, info);

        let lwork: i32 = T::xgetri_work_size(n_i32, &mut self_data[..], n_i32, &mut ipiv, &mut info);

        assert_eq!(0, info);

       	let mut work: Vec<T> = vec![T::zero(); lwork as usize];

        T::xgetri(n_i32, &mut self_data[..], n_i32, &mut ipiv, &mut work, lwork, &mut info);

        assert_eq!(0, info);


        let self_inv: Matrix<T> = Matrix::new(n, m, self_data);

        return Ok(self_inv.transpose_inplace());
    }
}