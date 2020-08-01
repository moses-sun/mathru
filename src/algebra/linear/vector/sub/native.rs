use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::Sub;

impl<T> Sub<T> for Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts a scalar value from all vector elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![6.0, 7.0, 8.0, 9.0]);
    ///
    /// assert_eq!(res_ref, a - -5.0)
    /// ```
    fn sub(mut self: Self, rhs: T) -> Self::Output
    {
        self.data = (&self.data).sub(&rhs);
        return self;
    }
}

impl<'a, T> Sub<&T> for &'a Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtract a scalar from vector elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, a - 5.0)
    /// ```
    fn sub(self: Self, rhs: &T) -> Self::Output
    {
        Vector { data: (&self.data).sub(rhs) }
    }
}

//c = a - b , a,b,c E T^m
impl<T> Sub for Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![-2.0, 6.0, -2.0, 0.0]);
    ///
    /// assert_eq!(res_ref, a - b)
    /// ```
    fn sub(self: Self, rhs: Vector<T>) -> Self::Output
    {
        (&self).sub(&rhs)
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![-2.0, 6.0, -2.0, 0.0]);
    ///
    /// assert_eq!(res_ref, &a - &b)
    /// ```
    fn sub(self: Self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector { data: (&self.data).sub(&rhs.data) }
    }
}
