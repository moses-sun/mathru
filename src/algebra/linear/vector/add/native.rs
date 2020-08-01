use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Vector},
};
use std::ops::Add;

impl<T> Add<Self> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![4.0, -2.0, 8.0, 8.0]);
    ///
    /// assert_eq!(res_ref, a + b)
    /// ```
    fn add(self: Self, rhs: Self) -> Self::Output
    {
        (&self).add(&rhs)
    }
}

impl<T> Add<T> for Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, a + -5.0)
    /// ```
    fn add(mut self: Self, rhs: T) -> Self::Output
    {
        self.data = (&self.data).add(&rhs);
        return self;
    }
}

impl<'a, T> Add<&T> for &'a Vector<T>
    where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
    ///
    /// assert_eq!(res_ref, &a + &-5.0)
    /// ```
    fn add(self: Self, rhs: &T) -> Self::Output
    {
        Vector { data: (&self.data).add(rhs) }
    }
}

//c = a + b, a,b,c E T^m
impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
    /// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
    /// let res_ref: Vector<f64> = Vector::new_column(4, vec![4.0, -2.0, 8.0, 8.0]);
    ///
    /// assert_eq!(res_ref, &a + &b)
    /// ```
    fn add(self: Self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector { data: (&self.data).add(&rhs.data) }
    }
}