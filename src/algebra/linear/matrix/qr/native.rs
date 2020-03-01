use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Field, Scalar};
use std::clone::Clone;
use serde::{Deserialize, Serialize};
use crate::elementary::Power;


/// QR decomposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRDec<T>
{
    q: Matrix<T>,
    r: Matrix<T>
}

impl<T> QRDec<T>
{
    pub(self) fn new(q: Matrix<T>, r: Matrix<T>) -> QRDec<T>
    {
        QRDec
        {
            q: q,
            r: r
        }
    }

    /// Return the q matrix of the QR decomposition
    ///
    /// # Arguments
    ///
    /// * `self`
    ///
    pub fn q(self: Self) -> Matrix<T>
    {
        return self.q;
    }

    /// Return the r matrix of the qr decomposition
    ///
    pub fn r(self: Self) -> Matrix<T>
    {
        return self.r;
    }

    pub fn qr(self: Self) -> (Matrix<T>, Matrix<T>)
    {
        return (self.q, self.r);
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar + Power
{
    /// QR Decomposition with Givens rotations
    ///
    /// A = QR \
    /// Q is an orthogonal matrix \
    /// R is an upper triangular matrix \
    ///
    /// # Panics
    ///
    /// if A is not a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    ///
    /// let (q, r): (Matrix<f64>, Matrix<f64>) = a.dec_qr().qr();
    ///
    /// ```
    pub fn dec_qr<'a>(self: &'a Self) -> QRDec<T>
    {
        let (m, n) = self.dim();
        assert!(m >= n);

        let mut q: Matrix<T> = Matrix::one(self.m);
        let mut r: Matrix<T> = self.clone();

        for j in 0..self.n
        {
            for i in (j + 1..self.m).rev()
            {
                let a_jj: T = *r.get(j, j);
                let a_ij: T = *r.get(i, j);
                //let k: T = a_jj.sgn();
                let p: T = (a_jj * a_jj + a_ij * a_ij).pow(&T::from_f64(0.5).unwrap());
                if (p != T::zero()) && (a_jj != T::zero()) && (a_ij != T::zero())
                {
                    let c: T = a_jj / p;
                    let s: T = -a_ij / p;
                    let g_ij: Matrix<T> = Matrix::givens(r.m, i, j, c, s);

                    r = &g_ij * &r;
                    q = &g_ij * &q;
                }
            }
        }
        q = q.transpose();
        return QRDec::new(q, r);
    }
}