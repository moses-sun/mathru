use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::lu::LUDec, Matrix},
};

pub trait Inverse<T>
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{matrix::Inverse, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    /// ```
    fn inv(self: &Self) -> Result<Matrix<T>, ()>;
}

impl<T> Inverse<T> for Matrix<T> where T: Field + Scalar
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{matrix::*, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    /// ```
    fn inv(self: &Self) -> Result<Matrix<T>, ()>
    {
        return self.inv_r();
    }
}

impl<T> Matrix<T> where T: Field + Scalar
{
    pub fn inv_r(self: &Self) -> Result<Matrix<T>, ()>
    {
        let lu_dec: LUDec<T> = self.dec_lu()?;
        return lu_dec.inv();
    }
}
