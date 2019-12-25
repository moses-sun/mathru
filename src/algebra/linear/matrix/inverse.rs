use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Real};

#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Zero};

#[cfg(feature = "native")]
use crate::algebra::linear::matrix::lu::{LUDec};


pub trait Inverse<T>
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    /// use mathru::algebra::linear::matrix::Inverse;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    ///
    /// ```
    fn inv(self: &Self) -> Option<Matrix<T>>;
}

impl<T> Inverse<T> for Matrix<T>
     where T: Real
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    /// use mathru::algebra::linear::matrix::*;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    ///
    /// ```
    fn inv(self: & Self) -> Option<Matrix<T>>
    {
        return self.inv_r();
    }
}

impl<T> Matrix<T>
    where T: Real
{
    #[cfg(feature = "native")]
    pub fn inv_r(self: &Self) -> Option<Matrix<T>>
    {
        let lu_dec: LUDec<T> = self.dec_lu();
        return lu_dec.inv();
    }

    #[cfg(feature = "blaslapack")]
    pub fn inv_r(self: & Self) -> Option<Matrix<T>>
    {
        let (m, n): (usize, usize) = self.dim();
        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;

        let dim_min: i32= m_i32.min(n_i32);
        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];

        let mut info: i32 = 0;

        let mut self_data: Vec<T> = self.clone().transpose().data;

        T::xgetrf(
            m_i32,
            n_i32,
            self_data.as_mut_slice(),
            m_i32,
            ipiv.as_mut_slice(),
            &mut info,
        );

        if info != 0
        {
            return None;
        }

        let lwork: i32 = T::xgetri_work_size(n_i32, &mut self_data[..], n_i32, &mut ipiv, &mut info);


        if info != 0
        {
            return None;
        }

       	let mut work: Vec<T> = vec![T::zero(); lwork as usize];

        T::xgetri(n_i32, &mut self_data[..], n_i32, &mut ipiv, &mut work, lwork, &mut info);

        if info != 0
        {
            return None;
        }

        let self_inv: Matrix<T> = Matrix::new(n, m, self_data);

        return Some(self_inv.transpose());
    }
}