use crate::algebra::linear::matrix::Diagonal;

impl<T> PartialEq for Diagonal<T>
where
    T: PartialEq,
{
    /// Checks if two diagonal matrices are equal
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, Diagonal};
    /// use mathru::matrix;
    ///
    /// let a: Diagonal<f64> = matrix![1.0, 0.0; 0.0, 5.0].into();
    /// let b: Diagonal<f64> = matrix![1.0, 0.0; 0.0, 5.0].into();
    ///
    /// assert_eq!(a ,b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.matrix.eq(&other.matrix)
    }
}
