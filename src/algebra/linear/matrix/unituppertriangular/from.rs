use crate::algebra::linear::matrix::{General, UnitUpperTriangular};

impl<T> From<General<T>> for UnitUpperTriangular<T> {
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, UnitUpperTriangular};
    /// use mathru::matrix;
    ///
    /// let m: UnitUpperTriangular<f64> = matrix![1.0, -2.0;
    ///                                           0.0, 1.0].into();
    /// ```
    fn from(matrix: General<T>) -> Self {
        UnitUpperTriangular { matrix }
    }
}
