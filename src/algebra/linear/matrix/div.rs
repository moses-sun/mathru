use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Field, Scalar};
use std::ops::{Div};

//Divides all  matrix elements with a scalar
impl<T> Div<T> for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Divides all matrix element with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let f: f64 = 7.0;
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![7.0, 0.0, 21.0, -49.0]);
    ///
    /// assert_eq!(res_ref, a / f);
    /// ```
    fn div(self, m: T) -> Matrix<T>
    {
        (&self).div(&m)
    }
}

impl<'a, 'b, T> Div<&'b T> for &'a Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;

    /// Divide all matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 0.0, 12.0, -28.0]);
    ///
    /// assert_eq!(res_ref, &a / &4.0);
    /// ```
    fn div(self, m: &'b T) -> Matrix<T>
    {
        return self.clone() * (T::one() / *m);
    }
}