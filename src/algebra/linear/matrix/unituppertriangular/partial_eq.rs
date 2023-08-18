use crate::algebra::linear::matrix::UnitUpperTriangular;

impl<T> PartialEq for UnitUpperTriangular<T>
where
    T: PartialEq,
{
    /// Checks if two matrices are equal
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, UnitUpperTriangular};
    /// use mathru::matrix;
    ///
    /// let a: UnitUpperTriangular<f64> = matrix![1.0, 0.0; 5.0, 1.0].into();
    /// let b: UnitUpperTriangular<f64> = matrix![1.0, 0.0; 5.0, 1.0].into();
    ///
    /// assert_eq!(true, a == b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.matrix.eq(&other.matrix)
    }
}
