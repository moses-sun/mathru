use crate::algebra::linear::matrix::{General, UpperTriangular};

impl<T> From<General<T>> for UpperTriangular<T> {
    ///
    /// # Example
    /// ```
    /// use mathru::algebra::linear::matrix::{General, UpperTriangular};
    /// use mathru::matrix;
    ///
    /// let m: UpperTriangular<f64> = matrix![1.0, -2.0;
    ///                                       0.0, -3.0].into();
    /// ```
    fn from(matrix: General<T>) -> Self {
        UpperTriangular { matrix }
    }
}
