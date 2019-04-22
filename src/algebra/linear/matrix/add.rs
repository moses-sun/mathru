use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Real};
use std::ops::{Add};

impl <T> Add for Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, Matrix::zero(2, 2) + a);
    /// ```
    fn add(self: Self, rhs: Self) -> Self::Output
    {
        (&self).add(&rhs)
    }
}


///
///Adds two matrices
///
impl <'a, 'b, T> Add<&'b Matrix<T>> for &'a Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, &Matrix::zero(2, 2) + &a);
    /// ```
    fn add(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());
        return self.add_r(rhs);
    }
}

impl<'a, 'b, T> Matrix<T>
    where T: Real
{
    #[cfg(feature = "native")]
    fn add_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let mut sum: Matrix<T> = self.clone();

        for i in 0..sum.m
        {
            for j in 0..sum.n
            {
                //*sum.get_mut(&i, &j) = self.get(&i, &j).clone() + rhs.get(&i, &j).clone();
                *sum.get_mut(&i, &j) += rhs.get(&i, &j).clone();
            }
        }
        sum
    }

    #[cfg(feature = "blaslapack")]
    fn add_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let mut c: Matrix<T> = rhs.clone();
        let (b_m, b_n): (usize, usize) = rhs.dim();

        let a: Matrix<T> = Matrix::one(b_m);
        let m: i32 = b_m as i32;
        let n: i32 = b_n as i32;
        let k: i32 = b_m as i32;

        T::xgemm('N' as u8, 'N' as u8, m, n, k, T::one(), &a.data[..], m, &self.data[..], k, T::one(), &mut c.data[..],
         m);

        return c;
    }
}