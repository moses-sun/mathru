use crate::algebra::linear::{Matrix};
use crate::elementary::Power;
use crate::algebra::abstr::{Field, Scalar};
use std::clone::Clone;
use serde::{Deserialize, Serialize};

/// Result of a cholesky decomposition
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CholeskyDec<T>
{
    l: Matrix<T>
}

impl<T> CholeskyDec<T>
    where T: Field + Scalar
{
    pub fn new(m: Matrix<T>) -> CholeskyDec<T>
    {
        CholeskyDec
        {
            l: m
        }
    }
}

impl<T> CholeskyDec<T>
{
    /// Return the l matrix
    pub fn l(self: Self) -> Matrix<T>
    {
        return self.l
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar + Power
{
    /// Decomposes the symetric, positive definite quadractic matrix A into a lower triangular matrix L
    /// A = L L^T
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
    pub fn dec_cholesky<'a>(self: &'a Self) -> Option<CholeskyDec<T>>
    {
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n);
        self.dec_cholesky_r()
    }

    #[cfg(feature = "blaslapack")]
    fn dec_cholesky_r<'a>(self: &'a Self) -> Option<CholeskyDec<T>>
    {
        let (_m, n) = self.dim();
        let n_i32: i32 = n as i32;

        let mut info: i32 = 0;

        let mut l_data: Vec<T> = self.clone().data;

        T::xpotrf(
            'L',
            n_i32,
            l_data.as_mut_slice(),
            n_i32,
            &mut info,
        );

        if info < 0
        {
            return None
        }
        //assert!(info >= 0);

        let mut l: Matrix<T> =  Matrix::new(n, n, l_data);

        //fill above diagonal with zeros
        for i in 0..n
        {
            for j in (i + 1)..n
            {
                *l.get_mut(i, j) = T::zero();
            }
        }

        return Some(CholeskyDec::new(l));
    }
}